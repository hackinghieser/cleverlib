use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

use crate::{
    clever_parser_options::CleverParserOptions, event::Event, event_collection::EventCollection,
};

pub struct ClefParser<'a> {
    file: File,
    pub lineCount: usize,
    path: &'a str,
    settings: ClefParserSettings,
    pub cached_chunks: VecDeque<Vec<Event>>,
    pub tail: usize,
    chunk_size: usize,
}

impl Clone for ClefParserSettings {
    fn clone(&self) -> Self {
        Self {
            chunk_size: self.chunk_size,
            ignore_errors: self.ignore_errors,
        }
    }
}
pub struct ClefParserSettings {
    pub chunk_size: usize,
    pub ignore_errors: bool,
}
impl<'a> ClefParser<'a> {
    pub fn new_with_defaults(path: &str) -> Result<ClefParser, io::Error> {
        let file = File::open(path)?;
        let buffer = BufReader::new(&file);
        let line_count = buffer.lines().count();
        Ok(ClefParser {
            path,
            file,
            cached_chunks: VecDeque::new(),
            tail: 1,
            lineCount: line_count,
            chunk_size: 1,
            settings: ClefParserSettings {
                chunk_size: 500,
                ignore_errors: true,
            },
        })
    }

    pub fn new(
        path: &str,
        chunk_size: usize,
        parser_settings: ClefParserSettings,
    ) -> Result<ClefParser, io::Error> {
        let file = File::open(path)?;
        let buffer = BufReader::new(&file);
        let line_count = buffer.lines().count();
        println!("Maxlines: {}", &line_count);
        Ok(ClefParser {
            path,
            file,
            lineCount: line_count,
            cached_chunks: VecDeque::new(),
            chunk_size,
            tail: 1,
            settings: parser_settings,
        })
    }

    pub fn get_next_chunk(&mut self) -> Option<Vec<Event>> {
        if self.tail > self.lineCount {
            return None;
        }

        println!(
            "Reading from line {} (chunk_size: {})",
            self.tail, self.chunk_size
        );

        let file = File::open(self.path).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .skip(self.tail - 1) // Convert 1-based to 0-based
            .take(self.chunk_size)
            .filter_map(|line| line.ok())
            .collect();

        println!("Read {} lines", lines.len());

        if lines.is_empty() {
            return None;
        }

        let events = EventCollection::create(
            &lines,
            Some(&CleverParserOptions {
                debug: Some(false),
                ignore_errors: Some(true),
            }),
        )
        .unwrap();

        // Add to cached_chunks and maintain max 3 chunks
        self.cached_chunks.push_back(events.events.clone());
        if self.cached_chunks.len() > 3 {
            self.cached_chunks.pop_front();
        }

        self.tail += self.chunk_size;
        Some(events.events)
    }

    pub fn cached_chunks_count(&self) -> usize {
        self.cached_chunks.len()
    }
    pub fn get_previous_chunk(&mut self) -> Option<Vec<Event>> {
        if self.tail <= 1 {
            return None;
        }

        // Move backward by chunk_size
        let new_tail = if self.tail > self.chunk_size {
            self.tail - self.chunk_size
        } else {
            1
        };

        println!(
            "Reading previous chunk from line {} (chunk_size: {})",
            new_tail, self.chunk_size
        );

        let file = File::open(self.path).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .skip(new_tail - 1) // Convert 1-based to 0-based
            .take(self.chunk_size)
            .filter_map(|line| line.ok())
            .collect();

        println!("Read {} lines", lines.len());

        if lines.is_empty() {
            return None;
        }

        let events = EventCollection::create(
            &lines,
            Some(&CleverParserOptions {
                debug: Some(false),
                ignore_errors: Some(true),
            }),
        )
        .unwrap();

        // Add to cached_chunks and maintain max 3 chunks
        self.cached_chunks.push_back(events.events.clone());
        if self.cached_chunks.len() > 3 {
            self.cached_chunks.pop_front();
        }

        self.tail = new_tail;
        Some(events.events)
    }
}
