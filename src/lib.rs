pub mod clefparser;
pub mod clever_parser_options;
pub mod errors;
pub mod event;
pub mod event_collection;
#[cfg(test)]
mod tests {
    use super::*;
    use clever_parser_options::CleverParserOptions;
    use event_collection::EventCollection;

    #[test]
    fn read_event() {
        let json_entries: [String; 1] = [
            r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"User {UserId} logged in from {IpAddress}","UserId":"user123","IpAddress":"192.168.1.1","@props":{"UserAgent":"Mozilla/5.0","SessionId":"sess_abc123"}}"#.to_string(),
        ];

        println!("{:?}", json_entries);
        let options = CleverParserOptions {
            ignore_errors: Some(true),
            debug: Some(true),
        };
        let collection = EventCollection::create(&json_entries, Some(&options)).unwrap();
        assert_eq!(collection.events.len(), 1)
    }

    #[test]
    fn read_multiple_events() {
        let json_entries: [String; 15] = [
            r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"User {UserId} logged in from {IpAddress}","UserId":"user123","IpAddress":"192.168.1.1","@props":{"UserAgent":"Mozilla/5.0","SessionId":"sess_abc123"}}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:29.456Z","@l":"Error","@mt":"Failed to process payment for order {OrderId}","@x":"System.Exception: Payment gateway timeout\n   at PaymentService.ProcessPayment()","OrderId":"ord_789","Amount":99.99,"Currency":"USD"}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:28.789Z","@l":"Warning","@mt":"Cache miss for key {CacheKey}","CacheKey":"user:123","@props":{"AttemptCount":3,"CacheSize":1024576}}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:27.234Z","@l":"Debug","@mt":"Database query executed in {ElapsedMilliseconds}ms","ElapsedMilliseconds":354,"@props":{"Query":"SELECT * FROM Users WHERE LastLogin > @date","Parameters":{"@date":"2024-12-27"}}}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:26.567Z","@l":"Information","@mt":"Order {OrderId} created for customer {CustomerId}","OrderId":"ord_790","CustomerId":"cust_456","@props":{"Items":3,"Total":150.00}}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:25.890Z","@l":"Error","@mt":"Authentication failed for user {Username}","@x":"AuthenticationException: Invalid credentials\n   at AuthService.Authenticate()","Username":"jdoe","@props":{"FailureCount":5,"LockoutEnabled":true}}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:24.123Z","@l":"Information","@mt":"API rate limit updated to {RequestsPerMinute} requests/minute","RequestsPerMinute":100,"@props":{"PlanType":"premium","ClientId":"client_789"}}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:23.456Z","@l":"Warning","@mt":"High memory usage detected: {MemoryUsageMB}MB","MemoryUsageMB":1567,"@props":{"ThresholdMB":1500,"ProcessId":1234}}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:22.789Z","@l":"Debug","@mt":"Cache entry expired for {Key} after {TimeToLiveSeconds}s","Key":"session:user:123","TimeToLiveSeconds":3600}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:21.234Z","@l":"Information","@mt":"Background job {JobId} completed","JobId":"job_123","@props":{"Duration":"00:05:23","ItemsProcessed":1500}}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:20.567Z","@l":"Error","@mt":"Failed to connect to database after {RetryCount} attempts","@x":"SqlException: Connection timeout\n   at DatabaseService.Connect()\n   at RetryPolicy.Execute()","RetryCount":3,"@props":{"Server":"db-prod-01","Port":5432}}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:19.890Z","@l":"Warning","@mt":"API endpoint {Endpoint} deprecated, use {NewEndpoint} instead","Endpoint":"/api/v1/users","NewEndpoint":"/api/v2/users"}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:18.123Z","@l":"Information","@mt":"Email notification {NotificationId} queued for {Recipients} recipients","NotificationId":"notif_456","Recipients":50,"@props":{"Template":"monthly_newsletter","ScheduledFor":"2024-12-29T09:00:00Z"}}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:17.456Z","@l":"Debug","@mt":"Request validation completed in {ElapsedMilliseconds}ms","ElapsedMilliseconds":45,"@props":{"ValidatedFields":["email","password","name"],"Errors":null}}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:16.789Z","@l":"Error","@mt":"File upload failed for user {UserId}","@x":"IOException: Insufficient disk space\n   at FileService.Upload()","UserId":"user_789","@props":{"FileName":"large_document.pdf","FileSize":15728640}}"#.to_string(),
        ];

        println!("{:?}", json_entries);
        let options = CleverParserOptions {
            ignore_errors: Some(true),
            debug: Some(true),
        };

        let collection = EventCollection::create(&json_entries, Some(&options)).unwrap();
        assert_eq!(collection.events.len(), 15)
    }

    #[test]
    fn read_event_without_timestamp() {
        let json_entries: [String; 1] = [
            r#"{"@l":"Information","@mt":"User {UserId} logged in from {IpAddress}","UserId":"user123","IpAddress":"192.168.1.1","@props":{"UserAgent":"Mozilla/5.0","SessionId":"sess_abc123"}}"#.to_string(),
        ];

        println!("{:?}", json_entries);
        let options = CleverParserOptions {
            ignore_errors: Some(true),
            debug: Some(true),
        };

        let collection = EventCollection::create(&json_entries, Some(&options)).unwrap();

        assert_eq!(collection.events.len(), 1);
        assert!(
            collection.events.first().unwrap().time.is_none(),
            "Timestamp found but not supplied"
        );
        assert_eq!(
            collection.events.first().unwrap().message.clone().unwrap(),
            "User user123 logged in from 192.168.1.1".to_string()
        );
    }

    #[test]
    fn read_event_without_log_level() {
        let json_entries: [String; 2] = [
            r#"{"@t":"2024-12-28T10:15:22.789Z","@mt":"Cache entry expired for {Key} after {TimeToLiveSeconds}s","Key":"session:user:123","TimeToLiveSeconds":3600}"#.to_string(),
            r#"{"@t":"2019-12-10T13:33:23.0970926Z","@mt":"An unknown error occurred","@l":"Error","@x":"Exception stack trace","SourceContext":"MySourceContext","Scope":["FirstScope",1850562557,"198441851"],"MachineName":"MY-MACHINE","EnvironmentUserName":"WORKGROUP\\SYSTEM","ExceptionDetail":{"HResult":-2146233087,"Message":"Internal Error","Source":"System.ServiceModel","Action":null,"Code":{"IsPredefinedFault":true,"IsSenderFault":true,"IsReceiverFault":false,"Namespace":"http://schemas.xmlsoap.org/soap/envelope/","Name":"Client","SubCode":null},"Reason":{"Translations":[{"XmlLang":"","Text":"Internal Error"}]},"Type":"System.ServiceModel.FaultException"},"AssemblyVersion":"2.1.0.1017"}"#.to_string()
        ];

        println!("{:?}", json_entries);
        let options = CleverParserOptions {
            ignore_errors: Some(true),
            debug: Some(true),
        };

        let collection = EventCollection::create(&json_entries, Some(&options)).unwrap();

        assert_eq!(collection.events.len(), 2);
        assert!(
            collection.events.first().unwrap().level.is_none(),
            "LogLevel set but not provided"
        );
        assert_eq!(
            collection.events.first().unwrap().message.clone().unwrap(),
            "Cache entry expired for session:user:123 after 3600s".to_string()
        );
    }

    #[test]
    fn read_event_with_mising_props() {
        let json_entries: [String; 2] = [
            r#"{"@t":"2024-12-28T10:15:22.789Z","@mt":"Cache entry expired for {Key} after {TimeToLiveSeconds}s","TimeToLiveSeconds":3600}"#.to_string(),
            r#"{"@t":"2019-12-10T13:33:23.0970926Z","@mt":"An unknown error occurred","@l":"Error","@x":"Exception stack trace","SourceContext":"MySourceContext","Scope":["FirstScope",1850562557,"198441851"],"MachineName":"MY-MACHINE","EnvironmentUserName":"WORKGROUP\\SYSTEM","ExceptionDetail":{"HResult":-2146233087,"Message":"Internal Error","Source":"System.ServiceModel","Action":null,"Code":{"IsPredefinedFault":true,"IsSenderFault":true,"IsReceiverFault":false,"Namespace":"http://schemas.xmlsoap.org/soap/envelope/","Name":"Client","SubCode":null},"Reason":{"Translations":[{"XmlLang":"","Text":"Internal Error"}]},"Type":"System.ServiceModel.FaultException"},"AssemblyVersion":"2.1.0.1017"}"#.to_string()
        ];

        println!("{:?}", json_entries);
        let options = CleverParserOptions {
            ignore_errors: Some(true),
            debug: Some(true),
        };

        let collection = EventCollection::create(&json_entries, Some(&options)).unwrap();

        assert_eq!(collection.events.len(), 2);
        assert_eq!(
            collection.events.first().unwrap().message.clone().unwrap(),
            "Cache entry expired for {Key} after 3600s".to_string()
        );
    }

    #[test]
    fn read_event_with_indexed_props() {
        let json_entries: [String; 1] = [
            r#"{"@t":"2024-12-28T10:15:22.789Z","@mt":"Cache entry expired for {0} after {1}s","Name":"Testname","TimeToLiveSeconds":3600}"#.to_string(),
        ];

        println!("{:?}", json_entries);
        let options = CleverParserOptions {
            ignore_errors: Some(false),
            debug: Some(true),
        };

        let collection = EventCollection::create(&json_entries, Some(&options)).unwrap();

        assert_eq!(collection.events.len(), 1);
        assert_eq!(
            collection.events.first().unwrap().message.clone().unwrap(),
            "Cache entry expired for Testname after 3600s".to_string()
        );
    }

    #[test]
    fn read_events_range() {
        let json_entries: [String; 5] = [
            r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"User {UserId} logged in from {IpAddress}","UserId":"user123","IpAddress":"192.168.1.1"}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:29.456Z","@l":"Error","@mt":"Failed to process payment for order {OrderId}","OrderId":"ord_789","Amount":99.99}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:28.789Z","@l":"Warning","@mt":"Cache miss for key {CacheKey}","CacheKey":"user:123"}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:27.234Z","@l":"Debug","@mt":"Database query executed in {ElapsedMilliseconds}ms","ElapsedMilliseconds":354}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:26.567Z","@l":"Information","@mt":"Order {OrderId} created for customer {CustomerId}","OrderId":"ord_790","CustomerId":"cust_456"}"#.to_string(),
        ];

        let options = CleverParserOptions {
            ignore_errors: Some(true),
            debug: Some(true),
        };

        // Test parsing range [1..4] (indices 1, 2, 3)
        let collection =
            EventCollection::create_range(&json_entries, 1, 4, Some(&options)).unwrap();

        assert_eq!(collection.events.len(), 3);
        assert_eq!(
            collection.events.first().unwrap().message.clone().unwrap(),
            "Failed to process payment for order ord_789".to_string()
        );
        assert_eq!(
            collection.events.last().unwrap().message.clone().unwrap(),
            "Database query executed in 354ms".to_string()
        );
    }

    #[test]
    fn read_events_range_full() {
        let json_entries: [String; 3] = [
            r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"User {UserId} logged in","UserId":"user123"}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:29.456Z","@l":"Error","@mt":"Payment failed for order {OrderId}","OrderId":"ord_789"}"#.to_string(),
            r#"{"@t":"2024-12-28T10:15:28.789Z","@l":"Warning","@mt":"Cache miss for {CacheKey}","CacheKey":"user:123"}"#.to_string(),
        ];

        let options = CleverParserOptions {
            ignore_errors: Some(true),
            debug: Some(true),
        };

        // Test parsing full range [0..3]
        let collection =
            EventCollection::create_range(&json_entries, 0, 3, Some(&options)).unwrap();

        assert_eq!(collection.events.len(), 3);
        assert_eq!(
            collection.events.first().unwrap().message.clone().unwrap(),
            "User user123 logged in".to_string()
        );
    }
}

#[cfg(test)]
mod clefparser_tests {
    use super::*;
    use clefparser::{ClefParser, ClefParserSettings};
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", content).unwrap();
        temp_file
    }

    fn create_empty_test_file() -> NamedTempFile {
        NamedTempFile::new().unwrap()
    }

    fn create_multi_line_test_file(lines: &[&str]) -> NamedTempFile {
        let mut temp_file = NamedTempFile::new().unwrap();
        for line in lines {
            writeln!(temp_file, "{}", line).unwrap();
        }
        temp_file
    }

    #[test]
    fn new_with_defaults_creates_parser() {
        let test_file = create_test_file(r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Test message"}"#);
        let path = test_file.path().to_str().unwrap();
        
        let parser = ClefParser::new_with_defaults(path);
        
        assert!(parser.is_ok());
        let parser = parser.unwrap();
        assert_eq!(parser.line_count, 1);
        assert_eq!(parser.tail, 1);
        assert_eq!(parser.cached_chunks_count(), 0);
    }

    #[test]
    fn new_with_defaults_fails_for_nonexistent_file() {
        let result = ClefParser::new_with_defaults("/nonexistent/file.clef");
        
        assert!(result.is_err());
        match result.unwrap_err() {
            errors::ClefParserError::FileOpenError { path, .. } => {
                assert_eq!(path, "/nonexistent/file.clef");
            }
            _ => panic!("Expected FileOpenError"),
        }
    }

    #[test]
    fn new_creates_parser_with_custom_settings() {
        let test_file = create_test_file(r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Test message"}"#);
        let path = test_file.path().to_str().unwrap();
        
        let settings = ClefParserSettings {
            chunk_size: 100,
            ignore_errors: false,
        };
        
        let parser = ClefParser::new(path, 5, settings);
        
        assert!(parser.is_ok());
        let parser = parser.unwrap();
        assert_eq!(parser.line_count, 1);
        assert_eq!(parser.chunk_size, 5);
    }

    #[test]
    fn new_fails_with_zero_chunk_size() {
        let test_file = create_test_file(r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Test message"}"#);
        let path = test_file.path().to_str().unwrap();
        
        let settings = ClefParserSettings {
            chunk_size: 100,
            ignore_errors: false,
        };
        
        let result = ClefParser::new(path, 0, settings);
        
        assert!(result.is_err());
        match result.unwrap_err() {
            errors::ClefParserError::InvalidChunkSize { size } => {
                assert_eq!(size, 0);
            }
            _ => panic!("Expected InvalidChunkSize error"),
        }
    }

    #[test]
    fn get_next_chunk_reads_single_event() {
        let test_file = create_test_file(r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"User {UserId} logged in","UserId":"user123"}"#);
        let path = test_file.path().to_str().unwrap();
        
        let mut parser = ClefParser::new_with_defaults(path).unwrap();
        
        let chunk = parser.get_next_chunk().unwrap();
        
        assert!(chunk.is_some());
        let events = chunk.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].message.as_ref().unwrap(), "User user123 logged in");
        assert_eq!(parser.cached_chunks_count(), 1);
    }

    #[test]
    fn get_next_chunk_reads_multiple_chunks() {
        let lines = [
            r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Line 1"}"#,
            r#"{"@t":"2024-12-28T10:15:31.123Z","@l":"Information","@mt":"Line 2"}"#,
            r#"{"@t":"2024-12-28T10:15:32.123Z","@l":"Information","@mt":"Line 3"}"#,
        ];
        let test_file = create_multi_line_test_file(&lines);
        let path = test_file.path().to_str().unwrap();
        
        let settings = ClefParserSettings {
            chunk_size: 2,
            ignore_errors: true,
        };
        let mut parser = ClefParser::new(path, 2, settings).unwrap();
        
        // First chunk should contain 2 events
        let chunk1 = parser.get_next_chunk().unwrap();
        assert!(chunk1.is_some());
        let events1 = chunk1.unwrap();
        assert_eq!(events1.len(), 2);
        
        // Second chunk should contain 1 event
        let chunk2 = parser.get_next_chunk().unwrap();
        assert!(chunk2.is_some());
        let events2 = chunk2.unwrap();
        assert_eq!(events2.len(), 1);
        
        // Third call should return None
        let chunk3 = parser.get_next_chunk().unwrap();
        assert!(chunk3.is_none());
    }

    #[test]
    fn get_next_chunk_returns_none_when_no_more_lines() {
        let test_file = create_test_file(r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Only line"}"#);
        let path = test_file.path().to_str().unwrap();
        
        let mut parser = ClefParser::new_with_defaults(path).unwrap();
        
        // First call should return the event
        let chunk1 = parser.get_next_chunk().unwrap();
        assert!(chunk1.is_some());
        
        // Second call should return None
        let chunk2 = parser.get_next_chunk().unwrap();
        assert!(chunk2.is_none());
    }

    #[test]
    fn get_previous_chunk_reads_backward() {
        let lines = [
            r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Line 1"}"#,
            r#"{"@t":"2024-12-28T10:15:31.123Z","@l":"Information","@mt":"Line 2"}"#,
            r#"{"@t":"2024-12-28T10:15:32.123Z","@l":"Information","@mt":"Line 3"}"#,
            r#"{"@t":"2024-12-28T10:15:33.123Z","@l":"Information","@mt":"Line 4"}"#,
        ];
        let test_file = create_multi_line_test_file(&lines);
        let path = test_file.path().to_str().unwrap();
        
        let settings = ClefParserSettings {
            chunk_size: 2,
            ignore_errors: true,
        };
        let mut parser = ClefParser::new(path, 2, settings).unwrap();
        
        // Move forward first
        parser.get_next_chunk().unwrap(); // Lines 1-2, tail becomes 3
        parser.get_next_chunk().unwrap(); // Lines 3-4, tail becomes 5
        
        // Now go backward - this will calculate new_tail = 5 - 2 = 3
        let chunk = parser.get_previous_chunk().unwrap();
        assert!(chunk.is_some());
        let events = chunk.unwrap();
        assert_eq!(events.len(), 2); // Should read lines 3-4 again
    }

    #[test]
    fn get_previous_chunk_returns_none_at_beginning() {
        let test_file = create_test_file(r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Only line"}"#);
        let path = test_file.path().to_str().unwrap();
        
        let mut parser = ClefParser::new_with_defaults(path).unwrap();
        
        // At the beginning, get_previous_chunk should return None
        let chunk = parser.get_previous_chunk().unwrap();
        assert!(chunk.is_none());
    }

    #[test]
    fn cached_chunks_maintains_max_three_chunks() {
        let lines = [
            r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Line 1"}"#,
            r#"{"@t":"2024-12-28T10:15:31.123Z","@l":"Information","@mt":"Line 2"}"#,
            r#"{"@t":"2024-12-28T10:15:32.123Z","@l":"Information","@mt":"Line 3"}"#,
            r#"{"@t":"2024-12-28T10:15:33.123Z","@l":"Information","@mt":"Line 4"}"#,
            r#"{"@t":"2024-12-28T10:15:34.123Z","@l":"Information","@mt":"Line 5"}"#,
        ];
        let test_file = create_multi_line_test_file(&lines);
        let path = test_file.path().to_str().unwrap();
        
        let mut parser = ClefParser::new_with_defaults(path).unwrap();
        
        // Read 4 chunks (each chunk contains 1 line with default settings)
        parser.get_next_chunk().unwrap();
        assert_eq!(parser.cached_chunks_count(), 1);
        
        parser.get_next_chunk().unwrap();
        assert_eq!(parser.cached_chunks_count(), 2);
        
        parser.get_next_chunk().unwrap();
        assert_eq!(parser.cached_chunks_count(), 3);
        
        parser.get_next_chunk().unwrap();
        assert_eq!(parser.cached_chunks_count(), 3); // Should still be 3, oldest chunk removed
    }

    #[test]
    fn clef_parser_settings_can_be_cloned() {
        let settings = ClefParserSettings {
            chunk_size: 100,
            ignore_errors: true,
        };
        
        let cloned_settings = settings.clone();
        
        assert_eq!(settings.chunk_size, cloned_settings.chunk_size);
        assert_eq!(settings.ignore_errors, cloned_settings.ignore_errors);
    }

    #[test]
    fn parser_counts_lines_correctly() {
        let lines = [
            r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Line 1"}"#,
            r#"{"@t":"2024-12-28T10:15:31.123Z","@l":"Information","@mt":"Line 2"}"#,
            r#"{"@t":"2024-12-28T10:15:32.123Z","@l":"Information","@mt":"Line 3"}"#,
        ];
        let test_file = create_multi_line_test_file(&lines);
        let path = test_file.path().to_str().unwrap();
        
        let parser = ClefParser::new_with_defaults(path).unwrap();
        
        assert_eq!(parser.line_count, 3);
    }

    #[test]
    fn parser_handles_empty_file() {
        let test_file = create_empty_test_file();
        let path = test_file.path().to_str().unwrap();
        
        let mut parser = ClefParser::new_with_defaults(path).unwrap();
        
        assert_eq!(parser.line_count, 0);
        let chunk = parser.get_next_chunk().unwrap();
        assert!(chunk.is_none());
    }

    #[test]
    fn parser_handles_invalid_json_with_ignore_errors() {
        let lines = [
            r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Valid line"}"#,
            r#"invalid json line"#,
            r#"{"@t":"2024-12-28T10:15:32.123Z","@l":"Information","@mt":"Another valid line"}"#,
        ];
        let test_file = create_multi_line_test_file(&lines);
        let path = test_file.path().to_str().unwrap();
        
        let settings = ClefParserSettings {
            chunk_size: 100,
            ignore_errors: true,
        };
        let mut parser = ClefParser::new(path, 10, settings).unwrap();
        
        let chunk = parser.get_next_chunk().unwrap();
        assert!(chunk.is_some());
        let events = chunk.unwrap();
        // Should only have 2 valid events, ignoring the invalid JSON
        assert_eq!(events.len(), 2);
    }

    #[test]
    fn parser_with_large_chunk_size_reads_all_lines() {
        let lines = [
            r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Line 1"}"#,
            r#"{"@t":"2024-12-28T10:15:31.123Z","@l":"Information","@mt":"Line 2"}"#,
        ];
        let test_file = create_multi_line_test_file(&lines);
        let path = test_file.path().to_str().unwrap();
        
        let settings = ClefParserSettings {
            chunk_size: 100,
            ignore_errors: true,
        };
        let mut parser = ClefParser::new(path, 100, settings).unwrap();
        
        let chunk = parser.get_next_chunk().unwrap();
        assert!(chunk.is_some());
        let events = chunk.unwrap();
        assert_eq!(events.len(), 2);
        
        // Next chunk should be None
        let chunk2 = parser.get_next_chunk().unwrap();
        assert!(chunk2.is_none());
    }

    #[test]
    fn parser_tail_position_updates_correctly() {
        let lines = [
            r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Line 1"}"#,
            r#"{"@t":"2024-12-28T10:15:31.123Z","@l":"Information","@mt":"Line 2"}"#,
            r#"{"@t":"2024-12-28T10:15:32.123Z","@l":"Information","@mt":"Line 3"}"#,
        ];
        let test_file = create_multi_line_test_file(&lines);
        let path = test_file.path().to_str().unwrap();
        
        let mut parser = ClefParser::new_with_defaults(path).unwrap();
        
        assert_eq!(parser.tail, 1);
        
        parser.get_next_chunk().unwrap();
        assert_eq!(parser.tail, 2); // Default chunk_size is 1
        
        parser.get_next_chunk().unwrap();
        assert_eq!(parser.tail, 3);
    }

    #[test]
    fn debug_implementation_works() {
        let test_file = create_test_file(r#"{"@t":"2024-12-28T10:15:30.123Z","@l":"Information","@mt":"Test message"}"#);
        let path = test_file.path().to_str().unwrap();
        
        let parser = ClefParser::new_with_defaults(path).unwrap();
        let debug_output = format!("{:?}", parser);
        
        assert!(debug_output.contains("ClefParser"));
        assert!(debug_output.contains("line_count"));
        assert!(debug_output.contains("tail"));
    }
}
