use std::fmt::Debug;

use chrono::DateTime;
use ratatui::{style::Style, widgets::{Cell, Row}};
use serde::Deserialize;
use serde_json::{self, Value};

#[derive(Deserialize, Debug, PartialEq)]
pub struct ClefLine<'a> {
    #[serde(rename = "@t")]
    #[serde(default)]
    time: String,

    #[serde(rename = "@m")]
    #[serde(default)]
    message: String,

    #[serde(rename = "@mt")]
    #[serde(default)]
    template: String,

    #[serde(rename = "@l")]
    #[serde(default)]
    level: String,

    #[serde(rename = "@x")]
    #[serde(default)]
    exception: String,

    #[serde(rename = "@i")]
    #[serde(default)]
    eventid: String,

    #[serde(rename = "@r")]
    #[serde(default)]
    renderings: String,

    #[serde(skip)]
    data: String,

    #[serde(skip)]
    pub row: Row<'a>,
}

impl<'a> ClefLine<'a> {

    pub fn new(line: &str) -> Self {
        let mut clef: ClefLine = serde_json::from_str(line).unwrap();
        clef.data = line.to_string();
        clef.template = clef.render();
        let time  = DateTime::parse_from_rfc3339(clef.time.as_str());
        clef.time = time.unwrap().format("%d.%m.%y %H:%M:%S").to_string();
        clef.row = Row::new(vec![
            Cell::from(clef.time.to_string()),
            Cell::from(clef.level.to_string()).style(Style::default().fg(ratatui::style::Color::Blue)),
            Cell::from(clef.template.to_string()),
        ]);
        clef
    }

    pub fn render(&mut self) -> String {
        let start_bracket = "{";
        let end_bracket = "}";
        let mut base = self.template.clone();
        let json: Value = serde_json::from_str(self.data.as_str()).unwrap();
        loop {
            let start = base.find(start_bracket).unwrap_or_default();
            let end = base.find(end_bracket).unwrap_or_default();
            if end == 0 {
                break;
            }
            let data_value = &base[start + 1..end];
            let json_value = &json[data_value];
            let mut template_value: String = String::from("empty");
            if let Some(number) = json_value.as_i64() {
                template_value = number.to_string();
            }

            if let Some(string) = json_value.as_str() {
                template_value = string.to_string();
            }
            base.replace_range(start..end + 1, template_value.as_str());
        }
   
        base
    }
}