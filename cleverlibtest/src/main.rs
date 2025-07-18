use cleverlib::clefpraser::{ClefParser, ClefParserSettings};
use std::fs;

fn generate_test_entries(count: usize) -> Vec<String> {
    let levels = ["Error", "Warning", "Info", "Debug"];
    let messages = [
        "File upload failed for user {UserId}",
        "Database connection timeout for {Service}",
        "User {Username} logged in successfully",
        "Cache miss for key {CacheKey}",
        "API request failed with status {StatusCode}",
    ];
    let exceptions = [
        "IOException: Insufficient disk space\\n   at FileService.Upload()",
        "SqlTimeoutException: Connection timeout\\n   at Database.Connect()",
        "NullPointerException: Object reference not set\\n   at Service.Process()",
        "ArgumentException: Invalid parameter\\n   at Validator.Check()",
    ];
    
    (0..count).map(|i| {
        let level = levels[i % levels.len()];
        let message = messages[i % messages.len()];
        let exception = if i % 3 == 0 { format!(r#","@x":"{}""#, exceptions[i % exceptions.len()]) } else { String::new() };
        let user_id = format!("user_{}", 100 + i);
        let timestamp = format!("2024-12-28T10:15:{:02}.789Z", 16 + (i % 44));
        
        format!(
            r#"{{"@t":"{}","@l":"{}","@mt":"{}","UserId":"{}","@props":{{"FileName":"document_{}.pdf","FileSize":{}}}{}}}"#,
            timestamp, level, message, user_id, i, 1024000 + i * 1000, exception
        )
    }).collect()
}

fn main() {
    println!("Testing cleverlib ClefParser chunking functionality...");

    // Create a test file with CLEF entries
    let test_file = "test_clef_entries.log";
    let test_entries = generate_test_entries(100);

    // Write test entries to file
    let content = test_entries.join("\n");
    if let Err(e) = fs::write(test_file, content) {
        eprintln!("Failed to create test file: {}", e);
        return;
    }

    println!("Created test file: {}", test_file);
    println!("Total entries: {}", test_entries.len());

    // Test ClefParser with custom settings - calling get_next_chunk 4 times
    println!("\nTesting ClefParser with custom settings - calling get_next_chunk 4 times...");
    let settings = ClefParserSettings {
        chunk_size: 25,
        ignore_errors: true,
    };

    match ClefParser::new(test_file, 25, settings.clone()) {
        Ok(mut parser) => {
            println!("✓ ClefParser created (chunk_size: 2)");
            let mut total_events = 0;

            let mut call_num = 0;
            loop {
                call_num += 1;
                println!("\n--- Call {} to get_next_chunk ---", call_num);

                if let Some(chunk) = parser.get_next_chunk() {
                    total_events += chunk.len();
                    println!(
                        "✓ Retrieved chunk with {} events (chunk size: {} bytes)",
                        chunk.len(),
                        std::mem::size_of_val(&chunk)
                    );
                    println!("  Total events processed so far: {}", total_events);
                    println!(
                        "  Cached chunks in VecDeque: {}",
                        parser.cached_chunks_count()
                    );
                } else {
                    println!("✗ No more chunks available");
                    println!("  Total events processed: {}", total_events);
                    println!(
                        "  Cached chunks in VecDeque: {}",
                        parser.cached_chunks_count()
                    );
                    println!("  VecDeque contents:");
                    for (i, cached_chunk) in parser.cached_chunks.iter().enumerate() {
                        println!(
                            "    Cached Chunk {}: {} events (size: {} bytes)",
                            i,
                            cached_chunk.len(),
                            std::mem::size_of_val(cached_chunk)
                        );
                    }
                    break;
                }
            }

            println!("\n=== Testing get_previous_chunk ===");
            println!(
                "Current tail position after forward navigation: {}",
                parser.tail
            );

            for call_num in 1..=4 {
                println!("\n--- Call {} to get_previous_chunk ---", call_num);

                if let Some(chunk) = parser.get_previous_chunk() {
                    println!(
                        "✓ Retrieved previous chunk with {} events (chunk size: {} bytes)",
                        chunk.len(),
                        std::mem::size_of_val(&chunk)
                    );
                    println!("  Current tail position: {}", parser.tail);
                    println!(
                        "  Cached chunks in VecDeque: {}",
                        parser.cached_chunks_count()
                    );

                    println!("  VecDeque contents:");
                    for (i, cached_chunk) in parser.cached_chunks.iter().enumerate() {
                        println!(
                            "    Cached Chunk {}: {} events (size: {} bytes)",
                            i,
                            cached_chunk.len(),
                            std::mem::size_of_val(cached_chunk)
                        );
                    }

                    println!(
                        "  Current previous chunk: {} events (size: {} bytes)",
                        chunk.len(),
                        std::mem::size_of_val(&chunk)
                    );
                } else {
                    println!("✗ No previous chunks available");
                    println!("  Current tail position: {}", parser.tail);
                    println!(
                        "  Cached chunks in VecDeque: {}",
                        parser.cached_chunks_count()
                    );
                }
            }

            println!("\n=== Final Summary ===");
            println!("Total lines in file: {}", parser.lineCount);
            println!("Total events processed: {}", total_events);
            println!("Final tail position: {}", parser.tail);
            println!(
                "Final cached chunks in VecDeque: {}",
                parser.cached_chunks_count()
            );
        }
        Err(e) => {
            println!("✗ Failed to create ClefParser: {}", e);
        }
    }

    // Clean up test file
    if let Err(e) = fs::remove_file(test_file) {
        eprintln!("Warning: Failed to remove test file: {}", e);
    }

    println!("\nClefParser chunking test completed!");
}
