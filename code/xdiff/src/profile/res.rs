use super::req::get_content_type;
use std::fmt::Write;

use anyhow::{Ok, Result};
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

impl ResponseProfile {
    pub fn new(skip_headers: Vec<String>, skip_body: Vec<String>) -> Self {
        Self {
            skip_headers,
            skip_body,
        }
    }
}

/// A Response Wrapper
#[derive(Debug)]
pub struct ResponseExt(pub Response);

impl ResponseExt {
    pub async fn get_text(self, profile: &ResponseProfile) -> Result<String> {
        let mut output = filter_headers(&self.0, &profile.skip_headers)?;

        let content_type = get_content_type(self.0.headers());
        let text = self.0.text().await?;
        match content_type.as_deref() {
            Some("application/json") => {
                let text = filter_json(&text, &profile.skip_body)?;
                writeln!(&mut output, "{}", text)?;
            }
            _ => {
                writeln!(&mut output, "{}", text)?;
            }
        }
        Ok(output)
    }

    pub fn get_header_keys(&self) -> Vec<String> {
        self.0
            .headers()
            .iter()
            .map(|(k, _)| k.to_string())
            .collect()
    }
}

pub fn filter_headers(res: &Response, skip_headers: &[String]) -> Result<String> {
    let mut output = String::new();
    writeln!(&mut output, "\n{:?} {:?}", res.version(), res.status())?;
    let headers = res.headers();

    for (k, v) in headers.iter() {
        if !skip_headers.contains(&k.to_string()) {
            writeln!(&mut output, "{}: {:?}", k, v)?;
        }
    }
    Ok(output)
}

pub fn filter_json(text: &str, skip_body: &[String]) -> Result<String> {
    let mut json: serde_json::Value = serde_json::from_str(text)?;
    if let serde_json::Value::Object(ref mut obj) = json {
        for k in skip_body {
            obj.remove(k);
        }
    }
    Ok(serde_json::to_string_pretty(&json)?)
}
