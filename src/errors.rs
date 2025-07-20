use thiserror::Error;

/// Errors that can occur when working with EventCollection
#[derive(Error, Debug)]
pub enum EventCollectionError {
    #[error("Failed to parse JSON event at index: {message}")]
    EventError {
        message: String,
    },

    #[error("Invalid range: start index {start} must be less than end index {end}")]
    InvalidRange { start: usize, end: usize },

    #[error("Range out of bounds: end index {end} exceeds collection length {length}")]
    RangeOutOfBounds { end: usize, length: usize },

    #[error("Empty event collection provided")]
    EmptyCollection,

    #[error("Regex compilation failed: {0}")]
    RegexError(#[from] regex::Error),

    #[error("Event processing failed: {message}")]
    ProcessingError { message: String },
}

#[derive(Error, Debug)]
pub enum EventError {

    #[error("Error generating the message template: {message}")]
    GenerateTemplateError { message: String}
}

/// Errors that can occur when working with ClefParser
#[derive(Error, Debug)]
pub enum ClefParserError {
    #[error("Failed to open file '{path}': {source}")]
    FileOpenError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read from file '{path}': {source}")]
    FileReadError {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid chunk size: {size} (must be greater than 0)")]
    InvalidChunkSize { size: usize },

    #[error("Invalid line position: {line} (file has {total_lines} lines)")]
    InvalidLinePosition { line: usize, total_lines: usize },

    #[error("End of file reached")]
    EndOfFile,

    #[error("No cached chunks available")]
    NoCachedChunks,

    #[error("Event collection error: {0}")]
    EventCollectionError(#[from] EventCollectionError),

    #[error("Parser initialization failed: {message}")]
    InitializationError { message: String },
}



/// A unified error type for the entire cleverlib crate
#[derive(Error, Debug)]
pub enum CleverLibError {
    #[error("Event collection error: {0}")]
    EventCollection(#[from] EventCollectionError),

    #[error("CLEF parser error: {0}")]
    ClefParser(#[from] ClefParserError),

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
}

/// Result type alias for EventCollection operations
pub type EventCollectionResult<T> = Result<T, EventCollectionError>;

/// Result type alias for ClefParser operations
pub type ClefParserResult<T> = Result<T, ClefParserError>;

/// Result type alias for general cleverlib operations
pub type CleverLibResult<T> = Result<T, CleverLibError>;
pub type EventResult<T> = Result<T, EventError>;
