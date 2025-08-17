# GEMINI.md

## Project Overview

This project is a Rust library named `cleverlib` for parsing and processing Compact Log Event Format (CLEF) structured log events. It is designed for high performance and flexibility, offering both serial and parallel processing strategies. The library can parse CLEF logs, identify log levels, and provides an API for filtering and analyzing log events.

**Key Technologies:**

- **Language:** Rust
- **Core Dependencies:**
  - `serde`: For JSON deserialization.
  - `regex`: For parsing log message templates.
  - `rayon`: For parallel processing.
  - `chrono`: For timestamp manipulation.
  - `indexmap`: For preserving the order of JSON properties.
  - `indicatif`: For progress tracking.
  - `thiserror`: For error handling.

**Architecture:**

The library is structured into several modules:

- `lib.rs`: The main library crate.
- `event.rs`: Defines the core `Event` struct, which represents a single log event.
- `event_collection.rs`: Manages collections of events and implements the processing strategies (serial and parallel).
- `clever_parser_options.rs`: Provides configuration options for the parser.
- `errors.rs`: Defines custom error types for the library.
- `clefparser.rs`: Implements the `ClefParser` for reading log files in chunks.

## Building and Running

**Building the library:**

```bash
cargo build
```

**Running tests:**

```bash
cargo test
```

## `cleverlibtest` Example

The `cleverlibtest` directory contains a test application that demonstrates how to use the `cleverlib` library. The example generates a test log file with 100 CLEF entries and then uses the `ClefParser` to read the file in chunks.

To run the example, navigate to the `cleverlibtest` directory and run the following command:

```bash
# From the cleverlibtest directory
cargo run
```

The example will print output to the console demonstrating the chunking functionality of the `ClefParser`.

**Dependencies:**

The `cleverlibtest` application has the following dependencies:

- `cleverlib`: The library itself (using a path dependency).
- `clap`: For command-line argument parsing.

## `ClefParser`

The `ClefParser` provides a way to read a CLEF log file in chunks, which is useful for large files. It also caches the chunks in a `VecDeque` to improve performance.

**Key Features:**

- **Chunking:** Reads a log file in chunks of a specified size.
- **Caching:** Caches the last 3 chunks in a `VecDeque` to improve performance.
- **Navigation:** Provides methods for getting the next and previous chunks.

## `Event` Struct

The `Event` struct is the central data structure in the library, representing a single log event. It uses `serde` for deserialization from JSON and has a custom `create` function that also handles the message template resolution.

**Key Features:**

- **Deserialization:** Deserializes a JSON log event into a structured `Event` object.
- **Message Template Resolution:** The `generate_message_template` function resolves the message template by interpolating properties from the event. It supports both named (`{PropertyName}`) and indexed (`{0}`) placeholders.

## `EventCollection` Struct

The `EventCollection` struct is the main entry point for processing log events. It provides methods for creating collections of events using serial, parallel, and callback-based processing strategies. It also provides methods for filtering events by log level.

**Key Features:**

- **Processing Strategies:** Supports serial, parallel, and callback-based processing of log events.
- **Filtering:** Provides methods for filtering events by log level.
- **Log Level Detection:** Automatically detects and stores unique log levels from the events.

## `CleverParserOptions` Struct

The `CleverParserOptions` struct allows the user to configure the parser's behavior.

**Fields:**

- `ignore_errors`: A boolean that determines whether the parser should ignore errors when parsing log events.
- `debug`: A boolean that enables or disables debug output.

## Development Conventions

- **Testing:** The project has a comprehensive test suite in the `lib.rs` file. The tests cover various scenarios, such as reading single and multiple events, handling events without timestamps or log levels, and processing events with missing or indexed properties. This gives a high level of confidence in the quality and correctness of the library.
- **Parallelism:** The library uses the Rayon crate for parallel processing of log events. The `EventCollection::create_par` function provides a parallel alternative to the default serial processing.
- **Error Handling:** The library uses the `thiserror` crate to define a set of custom error types. The main error types are `EventCollectionError`, `ClefParserError`, and `CleverLibError`. This provides a structured and descriptive way of handling errors that can occur during log processing.
- **Code Style:** The code follows standard Rust conventions and is formatted with `rustfmt`.
- **Commit Messages:** The project uses [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) to enforce a consistent commit message format. This is enforced by a GitHub Action that runs `commitlint` on all pushes and pull requests.
- **CI/CD:** The project has a CI/CD pipeline defined in the `.github/workflows/debug_build.yml` file. The pipeline includes jobs for generating a changelog, and for building and testing the project on Linux, Windows, and macOS. This ensures that the project is tested on all major platforms and that the changelog is kept up-to-date.
- **`.gitignore`:** The project has a standard Rust project `.gitignore` file, with the addition of `.idea` for JetBrains IDEs. It also ignores the `clevertest` directory, which is the test application.

