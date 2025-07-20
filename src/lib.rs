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
