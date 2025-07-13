use core::panic;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::{clever_parser_options::CleverParserOptions, event::Event};
use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::prelude::*;
use regex::Regex;

/// Represents a collection of log events with support for parallel and serial processing.
///
/// The `EventCollection` struct provides methods to create, filter, and manage log events
/// with various processing strategies including serial, parallel, and callback-based approaches.
#[derive(Default)]
pub struct EventCollection {
    /// A vector of parsed log events.
    pub events: Vec<Event>,
    /// A vector of unique log levels found in the events.
    pub log_levels: Vec<String>,
}

impl Debug for EventCollection {
    /// Custom debug implementation for `EventCollection`.
    ///
    /// This implementation simplifies the debug output to focus on the events.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventCollection")
            .field("events", &self.events)
            .finish()
    }
}

impl EventCollection {
    /// Creates an `EventCollection` using serial event processing.
    ///
    /// This method processes events sequentially, tracking progress and identifying unique log levels.
    ///
    /// # Arguments
    ///
    /// * `events` - A reference to a vector of log event strings
    ///
    /// # Returns
    ///
    /// An `Option<EventCollection>` containing the processed events
    pub fn create(
        events: &[String],
        options: Option<&CleverParserOptions>,
    ) -> Result<Self, serde_json::Error> {
        let mut event_collection = EventCollection {
            events: vec![],
            log_levels: vec![],
        };
        let event_list = EventCollection::read_events_serie(
            &mut event_collection,
            events,
            options.unwrap().ignore_errors.as_ref().unwrap().to_owned(),
        );
        event_collection.events = match event_list {
            Ok(value) => value,
            Err(e) => return Err(e),
        };
        Ok(event_collection)
    }

    /// Creates an `EventCollection` from a specific range of events.
    ///
    /// This method processes events from start index to end index (exclusive),
    /// similar to slice notation [start..end].
    ///
    /// # Arguments
    ///
    /// * `events` - A reference to a vector of log event strings
    /// * `start` - Starting index (inclusive)
    /// * `end` - Ending index (exclusive)
    /// * `options` - Optional parser configuration
    ///
    /// # Returns
    ///
    /// `Result<EventCollection, serde_json::Error>` containing the processed events from the range
    ///
    /// # Panics
    ///
    /// Panics if start >= end or if end > events.len()
    pub fn create_range(
        events: &[String],
        start: usize,
        end: usize,
        options: Option<&CleverParserOptions>,
    ) -> Result<Self, serde_json::Error> {
        if start >= end {
            panic!("Start index must be less than end index");
        }
        if end > events.len() {
            panic!("End index exceeds events length");
        }

        let event_slice = &events[start..end];
        let mut event_collection = EventCollection {
            events: vec![],
            log_levels: vec![],
        };
        let ignore_errors = options
            .and_then(|opts| opts.ignore_errors)
            .unwrap_or(false);
        
        let event_list = EventCollection::read_events_serie(
            &mut event_collection,
            event_slice,
            ignore_errors,
        );
        event_collection.events = match event_list {
            Ok(value) => value,
            Err(e) => return Err(e),
        };
        Ok(event_collection)
    }

    /// Creates an `EventCollection` using parallel event processing.
    ///
    /// This method processes events concurrently using Rayon's parallel iterator.
    ///
    /// # Arguments
    ///
    /// * `events` - A reference to a vector of log event strings
    ///
    /// # Returns
    ///
    /// An `Option<EventCollection>` containing the processed events
    pub fn create_par(events: &Vec<String>) -> Option<Self> {
        let mut event_collection = EventCollection {
            events: vec![],
            log_levels: vec![],
        };
        let event_list = EventCollection::read_events_par(&mut event_collection, events);
        event_collection.events = event_list;
        Some(event_collection)
    }

    /// Creates an `EventCollection` with a custom progress callback.
    ///
    /// This method allows for custom progress tracking during event processing.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A closure that can be called with a `usize` parameter for progress tracking
    ///
    /// # Arguments
    ///
    /// * `events` - A vector of log event strings
    /// * `callback` - A closure to track processing progress
    ///
    /// # Returns
    ///
    /// An `Option<EventCollection>` containing the processed events
    pub fn create_cp<T: Fn(usize) + Send + Sync + 'static>(
        events: Vec<String>,
        callback: T,
    ) -> Option<Self> {
        let mut event_collection = EventCollection {
            events: vec![],
            log_levels: vec![],
        };
        let event_list = EventCollection::read_events_cp(&event_collection, events, callback);
        event_collection.events = event_list;
        Some(event_collection)
    }

    /// Internal method to process events with a custom progress callback.
    ///
    /// # Safety and Performance
    ///
    /// Uses Rayon's parallel iterator for concurrent event processing.
    ///
    /// # Arguments
    ///
    /// * `events` - A vector of log event strings
    /// * `callback` - A closure for progress tracking
    ///
    /// # Returns
    ///
    /// A vector of processed `Event` instances
    fn read_events_cp<T: Fn(usize) + Send + Sync + 'static>(
        &self,
        events: Vec<String>,
        callback: T,
    ) -> Vec<Event> {
        let re = Regex::new(r"\{(\w+|\d+)\}").unwrap();
        let event_collection = events
            .par_iter()
            .enumerate()
            .map(|e| {
                callback(0);
                Event::create(e.1.to_string(), &re).unwrap()
            })
            .collect();
        event_collection
    }

    /// Filters events by log level in parallel using case-insensitive comparison.
    ///
    /// # Arguments
    ///
    /// * `log_level` - The log level to filter by
    ///
    /// # Returns
    ///
    /// A vector of references to `Event` instances matching the specified log level
    pub fn filter_log_level_par(&self, log_level: &str) -> Vec<&Event> {
        self.events
            .par_iter()
            .filter(|event| event.level.clone().unwrap().eq_ignore_ascii_case(log_level))
            .collect()
    }

    /// Filters events by log level in parallel with a fallback for events without a log level.
    ///
    /// # Arguments
    ///
    /// * `log_level` - The log level to filter by
    ///
    /// # Returns
    ///
    /// A vector of references to `Event` instances matching the specified log level
    pub fn filter_log_level(&self, log_level: &str) -> Vec<&Event> {
        self.events
            .par_iter()
            .filter(|event| {
                event
                    .level
                    .clone()
                    .unwrap_or_default()
                    .eq_ignore_ascii_case(log_level)
            })
            .collect()
    }

    /// Internal method to read events serially with progress tracking.
    ///
    /// # Arguments
    ///
    /// * `events` - A slice of log event strings
    ///
    /// # Returns
    ///
    /// A vector of processed `Event` instances with tracked log levels
    fn read_events_serie(
        &mut self,
        events: &[String],
        ignore_erros: bool,
    ) -> Result<Vec<Event>, serde_json::Error> {
        if ignore_erros {
            let re = Regex::new(r"\{(\w+|\d+)\}").unwrap();
            let mut log_levels: Vec<String> = vec![];
            let event_collection = events
                .iter()
                .progress()
                .filter_map(|e| match Event::create(e.to_string(), &re) {
                    Ok(event) => {
                        if let Some(level) = event.level.clone() {
                            if !log_levels.contains(&level) {
                                log_levels.push(level)
                            }
                        }
                        Some(event)
                    }
                    Err(err) => {
                        println!("Could not parse event: {}, Error: {}", e, err);
                        None
                    }
                })
                .collect::<Vec<Event>>();
            self.log_levels = log_levels;
            Ok(event_collection)
        } else {
            let re = Regex::new(r"\{(\w+|\d+)\}").unwrap();
            let mut log_levels: Vec<String> = vec![];
            let mut index: i128 = 1;
            let event_collection = events
                .iter()
                .progress()
                .filter_map(|e| match Event::create(e.to_string(), &re) {
                    Ok(event) => {
                        if let Some(level) = event.level.clone() {
                            if !log_levels.contains(&level) {
                                log_levels.push(level)
                            }
                        }
                        index += 1;
                        Some(event)
                    }
                    Err(err) => {
                        panic!("Error parsing event no. {}\r\n{}", index, err)
                    }
                })
                .collect();
            self.log_levels = log_levels;
            Ok(event_collection)
        }
    }

    /// Internal method to read events in parallel with progress tracking.
    ///
    /// # Arguments
    ///
    /// * `events` - A reference to a vector of log event strings
    ///
    /// # Returns
    ///
    /// A vector of processed `Event` instances with tracked log levels
    fn read_events_par(&mut self, events: &Vec<String>) -> Vec<Event> {
        let re = Regex::new(r"\{(\w+|\d+)\}").unwrap();
        let log_levels: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
        let event_collection = events
            .par_iter()
            .progress()
            .map(|e| {
                let e = Event::create(e.to_string(), &re).unwrap();
                if let Some(level) = e.level.clone() {
                    log_levels.lock().unwrap().push(level)
                }
                e
            })
            .collect();
        self.log_levels = Arc::try_unwrap(log_levels).unwrap().into_inner().unwrap();
        event_collection
    }
}
