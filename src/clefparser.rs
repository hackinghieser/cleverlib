use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    clever_parser_options::CleverParserOptions, 
    errors::{ClefParserError, ClefParserResult},
    event::Event, 
    event_collection::EventCollection,
};

pub struct ClefParser<'a> {
    file: File,
    pub line_count: usize,
    path: &'a str,
    settings: ClefParserSettings,
    pub cached_chunks: VecDeque<Vec<Event>>,
    pub tail: usize,
    pub chunk_size: usize,
}

impl<'a> std::fmt::Debug for ClefParser<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClefParser")
            .field("line_count", &self.line_count)
            .field("path", &self.path)
            .field("settings", &self.settings)
            .field("cached_chunks", &self.cached_chunks)
            .field("tail", &self.tail)
            .field("chunk_size", &self.chunk_size)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct ClefParserSettings {
    pub chunk_size: usize,
    pub ignore_errors: bool,
}
impl<'a> ClefParser<'a> {
    pub fn new_with_defaults(path: &str) -> ClefParserResult<ClefParser> {
        let file = File::open(path).map_err(|e| ClefParserError::FileOpenError {
            path: path.to_string(),
            source: e,
        })?;
        let buffer = BufReader::new(&file);
        let line_count = buffer.lines().count();
        Ok(ClefParser {
            path,
            file,
            cached_chunks: VecDeque::new(),
            tail: 1,
            line_count,
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
    ) -> ClefParserResult<ClefParser> {
        if chunk_size == 0 {
            return Err(ClefParserError::InvalidChunkSize { size: chunk_size });
        }
        let file = File::open(path).map_err(|e| ClefParserError::FileOpenError {
            path: path.to_string(),
            source: e,
        })?;
        let buffer = BufReader::new(&file);
        let line_count = buffer.lines().count();
        println!("Maxlines: {}", &line_count);
        Ok(ClefParser {
            path,
            file,
            line_count,
            cached_chunks: VecDeque::new(),
            chunk_size,
            tail: 1,
            settings: parser_settings,
        })
    }

    pub fn get_next_chunk(&mut self) -> ClefParserResult<Option<Vec<Event>>> {
        if self.tail > self.line_count {
            return Ok(None);
        }

        println!(
            "Reading from line {} (chunk_size: {})",
            self.tail, self.chunk_size
        );

        let file = File::open(self.path).map_err(|e| ClefParserError::FileOpenError {
            path: self.path.to_string(),
            source: e,
        })?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .skip(self.tail - 1) // Convert 1-based to 0-based
            .take(self.chunk_size)
            .filter_map(|line| line.ok())
            .collect();

        println!("Read {} lines", lines.len());

        if lines.is_empty() {
            return Ok(None);
        }

        let events = EventCollection::create(
            &lines,
            Some(&CleverParserOptions {
                debug: Some(false),
                ignore_errors: Some(true),
            }),
        )?;

        // Add to cached_chunks and maintain max 3 chunks
        self.cached_chunks.push_back(events.events.clone());
        if self.cached_chunks.len() > 3 {
            self.cached_chunks.pop_front();
        }

        self.tail += self.chunk_size;
        Ok(Some(events.events))
    }

    pub fn cached_chunks_count(&self) -> usize {
        self.cached_chunks.len()
    }
    pub fn get_previous_chunk(&mut self) -> ClefParserResult<Option<Vec<Event>>> {
        if self.tail <= 1 {
            return Ok(None);
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

        let file = File::open(self.path).map_err(|e| ClefParserError::FileOpenError {
            path: self.path.to_string(),
            source: e,
        })?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .skip(new_tail - 1) // Convert 1-based to 0-based
            .take(self.chunk_size)
            .filter_map(|line| line.ok())
            .collect();

        println!("Read {} lines", lines.len());

        if lines.is_empty() {
            return Ok(None);
        }

        let events = EventCollection::create(
            &lines,
            Some(&CleverParserOptions {
                debug: Some(false),
                ignore_errors: Some(true),
            }),
        )?;

        // Add to cached_chunks and maintain max 3 chunks
        self.cached_chunks.push_back(events.events.clone());
        if self.cached_chunks.len() > 3 {
            self.cached_chunks.pop_front();
        }

        self.tail = new_tail;
        Ok(Some(events.events))
    }
}
