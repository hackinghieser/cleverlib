use indexmap::IndexMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a structured log event with rich metadata and template parsing capabilities.
///
/// The `Event` struct is designed to parse and process complex log events with dynamic properties,
/// supporting flexible JSON deserialization and message template resolution.
///
/// # Fields
///
/// * `time` - Optional timestamp of the event
/// * `message` - Resolved message with properties interpolated
/// * `template` - Original message template
/// * `level` - Optional log level
/// * `exception` - Optional exception details
/// * `eventid` - Optional event identifier
/// * `renderings` - Vector of rendering information
/// * `data` - Additional event data as a string
/// * `properties` - Dynamic properties of the event
/// * `raw_string` - Original raw event string
#[derive(Deserialize, Debug, PartialEq, Serialize)]
pub struct Event {
    /// Timestamp of the event
    #[serde(rename = "@t")]
    #[serde(default)]
    pub time: Option<String>,

    /// Resolved message with interpolated properties
    #[serde(rename = "@m")]
    #[serde(default)]
    pub message: Option<String>,

    /// Original message template
    #[serde(rename = "@mt")]
    #[serde(default)]
    pub template: String,

    /// Log level of the event
    #[serde(rename = "@l")]
    #[serde(default)]
    pub level: Option<String>,

    /// Exception details, if any
    #[serde(rename = "@x")]
    #[serde(default)]
    pub exception: Option<String>,

    /// Unique event identifier
    #[serde(rename = "@i")]
    #[serde(default)]
    pub eventid: Option<String>,

    /// Renderings associated with the event
    #[serde(rename = "@r")]
    #[serde(default)]
    pub renderings: Vec<String>,

    /// Additional event data
    #[serde(skip)]
    pub data: String,

    /// Dynamic properties of the event
    #[serde(flatten)]
    properties: IndexMap<String, Value>,

    /// Original raw event string
    #[serde(skip)]
    raw_string: String,
}

impl Event {
    /// Creates an `Event` from a raw JSON event string.
    ///
    /// This method parses the JSON, deserializes the event, and resolves the message template
    /// by interpolating properties from the event.
    ///
    /// # Arguments
    ///
    /// * `raw_event` - A JSON-formatted event string
    /// * `regex` - A regular expression for parsing template placeholders
    ///
    /// # Returns
    ///
    /// An `Option<Event>` containing the parsed and processed event
    ///
    /// # Examples
    ///
    /// ```
    /// use cleverlib::event::Event;
    /// use regex::Regex;
    ///
    /// let json_event = r#"{"@t": "2024-01-15", "@mt": "User {Username} logged in", "Username": "john_doe"}"#;
    /// let regex = Regex::new(r"\{(\w+)\}").unwrap();
    /// let event = Event::create(json_event.to_string(), &regex);
    /// ```
    pub fn create(raw_event: String, regex: &Regex) -> Result<Self, serde_json::Error> {
        let raw_json: Value = serde_json::from_str(raw_event.as_str())?;
        let mut event: Event = serde_json::from_value(raw_json.clone())?;
        event.raw_string = raw_event;
        event.message = Some(Event::generate_message_template(
            &event.template,
            &event.properties,
            regex,
        ));
        Ok(event)
    }

    /// Generates a resolved message by interpolating template placeholders with event properties.
    ///
    /// This method supports placeholders using both named and indexed property references.
    ///
    /// # Arguments
    ///
    /// * `template` - The original message template
    /// * `properties` - A map of event properties
    /// * `regex` - A regular expression for identifying placeholders
    ///
    /// # Returns
    ///
    /// A `String` with placeholders replaced by their corresponding property values
    ///
    /// # Notes
    ///
    /// - Supports both `{PropertyName}` and `{0}` style placeholders
    /// - Unresolved placeholders are left in their original format
    ///
    /// # Examples
    ///
    /// ```
    /// use indexmap::IndexMap;
    /// use serde_json::json;
    /// use cleverlib::event::Event;
    /// use regex::Regex;
    ///
    /// let template = "User {Username} performed {Action}";
    /// let mut properties = IndexMap::new();
    /// properties.insert("Username".to_string(), json!("john_doe"));
    /// properties.insert("Action".to_string(), json!("login"));
    ///
    /// let regex = Regex::new(r"\{(\w+)\}").unwrap();
    /// let resolved = Event::generate_message_template(template, &properties, &regex);
    /// // resolved will be "User john_doe performed login"
    /// ```
    pub fn generate_message_template(
        template: &str,
        properties: &IndexMap<String, Value>,
        regex: &Regex,
    ) -> String {
        let res = regex
            .replace_all(template, |caps: &regex::Captures| {
                let key = &caps[1];
                if let Ok(index) = key.parse::<usize>() {
                    properties
                        .get_index(index)
                        .map_or(format!("{{{}}}", index), |v| match v.1 {
                            Value::String(v) => v.as_str().to_string(),
                            Value::Number(v) => v.to_string(),
                            Value::Bool(v) => v.to_string(),
                            _ => v.1.to_string(),
                        })
                } else {
                    properties
                        .get(key)
                        .map_or(format!("{{{}}}", key), |v| match v {
                            Value::String(v) => v.as_str().to_string(),
                            Value::Number(v) => v.to_string(),
                            Value::Bool(v) => v.to_string(),
                            _ => v.to_string(),
                        })
                }
            })
            .to_string();
        res
    }
}
