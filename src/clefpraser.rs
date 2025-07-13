use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::{
    clever_parser_options::CleverParserOptions, event::Event, event_collection::EventCollection,
};

pub struct ClefParser<'a> {
    file: File,
    path: &'a str,
    settings: ClefParserSettings,
    cached_chunks: VecDeque<Vec<Event>>,
    tail: usize,
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
    chunk_size: usize,
    ignore_errors: bool,
}
impl<'a> ClefParser<'a> {
    fn new_with_defaults(path: &str) -> Result<ClefParser, io::Error> {
        let file = File::open(path)?;
        Ok(ClefParser {
            path,
            file,
            cached_chunks: VecDeque::new(),
            tail: 0,
            chunk_size: 500,
            settings: ClefParserSettings {
                chunk_size: 500,
                ignore_errors: true,
            },
        })
    }

    fn new(
        path: &str,
        chunk_size: usize,
        parser_settings: ClefParserSettings,
    ) -> Result<ClefParser, io::Error> {
        let file = File::open(path)?;
        Ok(ClefParser {
            path,
            file,
            cached_chunks: VecDeque::new(),
            chunk_size,
            tail: 0,
            settings: parser_settings,
        })
    }

    fn get_next_chunk(mut self) -> VecDeque<Vec<Event>> {
        let reader = BufReader::new(self.file);
        let end = self.tail + self.chunk_size;
        let lines: Vec<String> = reader
            .lines()
            .enumerate()
            .skip_while(|(index, _)| *index < self.tail - 1)
            .take_while(|(index, _)| *index < end)
            .filter_map(|(_, line)| line.ok())
            .collect();
        let events = EventCollection::create(
            &lines,
            Some(&CleverParserOptions {
                debug: Some(false),
                ignore_errors: Some(true),
            }),
        )
        .unwrap();
        self.cached_chunks.push_back(events.events);
        self.cached_chunks
    }
    fn get_previous_chunk(mut self) -> VecDeque<Vec<Event>> {
        let reader = BufReader::new(self.file);
        let end = self.tail + self.chunk_size;
        let lines: Vec<String> = reader
            .lines()
            .enumerate()
            .skip_while(|(index, _)| *index < self.tail - 1)
            .take_while(|(index, _)| *index < end)
            .filter_map(|(_, line)| line.ok())
            .collect();
        let events = EventCollection::create(
            &lines,
            Some(&CleverParserOptions {
                debug: Some(false),
                ignore_errors: Some(true),
            }),
        )
        .unwrap();
        self.cached_chunks.push_back(events.events);
        self.cached_chunks
    }
}
