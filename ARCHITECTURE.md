# CleverLib Architecture

## Overview

CleverLib is a high-performance Rust library designed for parsing and processing Compact Log Event Format (CLEF) structured log events. The library provides flexible processing strategies including serial and parallel execution, with comprehensive support for log event templating and property interpolation.

## Core Architecture

### Module Structure

```
cleverlib/
├── src/
│   ├── lib.rs                      # Library entry point and module declarations
│   ├── event.rs                    # Core Event struct and message templating
│   ├── event_collection.rs         # EventCollection with processing strategies
│   └── clever_parser_options.rs    # Configuration options for parsing
├── Cargo.toml                      # Project metadata and dependencies
└── README.md                       # User documentation
```

### Key Components

#### 1. Event (`event.rs`)

The `Event` struct represents a structured log event with rich metadata and template parsing capabilities.

**Core Fields:**
- `time`: Optional timestamp (maps to `@t` in CLEF)
- `template`: Message template with placeholders (maps to `@mt`)
- `message`: Resolved message with interpolated properties (maps to `@m`)
- `level`: Log level (maps to `@l`)
- `exception`: Exception details (maps to `@x`)
- `eventid`: Event identifier (maps to `@i`)
- `renderings`: Rendering information (maps to `@r`)
- `properties`: Dynamic key-value properties (flattened from JSON)

**Key Features:**
- JSON deserialization with serde
- Template message resolution with regex-based placeholder replacement
- Support for both named (`{PropertyName}`) and indexed (`{0}`) placeholders
- Preservation of raw event data

#### 2. EventCollection (`event_collection.rs`)

The `EventCollection` struct manages collections of events with multiple processing strategies.

**Processing Strategies:**
- **Serial Processing** (`create`): Sequential event processing with progress tracking
- **Parallel Processing** (`create_par`): Concurrent processing using Rayon
- **Callback Processing** (`create_cp`): Custom progress callback support

**Key Features:**
- Automatic log level detection and aggregation
- Parallel and serial filtering by log level
- Progress tracking with indicatif
- Error handling with configurable ignore options
- Thread-safe parallel processing

#### 3. CleverParserOptions (`clever_parser_options.rs`)

Configuration struct for parsing behavior:
- `ignore_errors`: Continue processing on parse errors
- `debug`: Enable debug output

## Design Patterns

### 1. Builder Pattern
The library uses factory methods (`create`, `create_par`, `create_cp`) to construct `EventCollection` instances with different processing strategies.

### 2. Strategy Pattern
Multiple processing strategies are implemented:
- Serial processing for smaller datasets
- Parallel processing for large datasets
- Callback-based processing for custom progress tracking

### 3. Template Method Pattern
The `Event::generate_message_template` method provides a template for message resolution that can handle various placeholder formats.

## Data Flow

```
Raw JSON Log Lines
        ↓
[EventCollection::create]
        ↓
[Regex-based parsing]
        ↓
[Event::create for each line]
        ↓
[Template resolution]
        ↓
[Property interpolation]
        ↓
Structured Event Collection
```

## Performance Considerations

### Parallelization
- Uses Rayon for data parallelism
- Parallel iterator support for large datasets
- Thread-safe log level aggregation with Arc<Mutex<>>

### Memory Management
- IndexMap for ordered property preservation
- Efficient regex compilation (compiled once, reused)
- Optional progress tracking to avoid overhead

### Error Handling
- Configurable error handling (ignore vs. panic)
- Graceful degradation with filter_map for error cases
- Detailed error reporting with line numbers

## Dependencies

### Core Dependencies
- **serde**: JSON serialization/deserialization with derive macros
- **serde_json**: JSON parsing with value preservation
- **indexmap**: Ordered map implementation for properties
- **regex**: Pattern matching for template placeholders
- **chrono**: Date/time handling

### Performance Dependencies
- **rayon**: Data parallelism
- **rayon-progress**: Progress tracking for parallel operations
- **indicatif**: Progress bars and indicators

## CLEF Format Support

The library fully supports the Compact Log Event Format (CLEF) specification:

- `@t`: Timestamp
- `@mt`: Message template
- `@m`: Rendered message
- `@l`: Log level
- `@x`: Exception
- `@i`: Event ID
- `@r`: Renderings
- `@props`: Additional properties (flattened)

## Testing Strategy

The library includes comprehensive tests covering:
- Basic event parsing
- Multiple event processing
- Events without timestamps
- Events without log levels
- Missing properties handling
- Indexed property placeholders
- Error handling scenarios

## Usage Patterns

### Basic Usage
```rust
let events = vec![json_log_line1, json_log_line2];
let collection = EventCollection::create(&events, Some(&options))?;
```

### Parallel Processing
```rust
let collection = EventCollection::create_par(&events).unwrap();
```

### Filtering
```rust
let error_events = collection.filter_log_level("error");
```

## Extension Points

The architecture supports extension through:
- Custom processing strategies via new factory methods
- Additional event fields through serde attributes
- Custom template resolution logic
- Pluggable progress tracking systems

## Performance Characteristics

- **Memory**: O(n) where n is the number of events
- **Time Complexity**: O(n) for serial, O(n/p) for parallel where p is thread count
- **Regex Performance**: Compiled once, reused across all events
- **Thread Safety**: Full thread safety for parallel operations

This architecture provides a robust foundation for high-performance log processing while maintaining flexibility and extensibility.