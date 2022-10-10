use super::res::ResponseExt;
use crate::cli::OverrideArgs;

use std::str::FromStr;

use anyhow::{anyhow, Ok, Result};
use http::header;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Method,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,

    pub url: Url,

    #[serde(skip_serializing_if = "is_empty_json_value", default)]
    pub params: Option<serde_json::Value>,

    #[serde(
        skip_serializing_if = "HeaderMap::is_empty",
        with = "http_serde::header_map",
        default
    )]
    pub headers: HeaderMap,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<serde_json::Value>,
}

fn is_empty_json_value(v: &Option<serde_json::Value>) -> bool {
    v.as_ref().map_or(true, |v| {
        v.is_null() || (v.is_object() && v.as_object().unwrap().is_empty())
    })
}

impl FromStr for RequestProfile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut url = Url::parse(s)?;
        let query = url.query_pairs();
        let mut params = json!({});
        for (k, v) in query {
            params[k.as_ref()] = v.parse()?;
        }
        url.set_query(None);

        Ok(RequestProfile::new(
            Method::GET,
            url,
            Some(params),
            HeaderMap::default(),
            None,
        ))
    }
}

impl RequestProfile {
    pub fn new(
        method: Method,
        url: Url,
        params: Option<serde_json::Value>,
        headers: HeaderMap,
        body: Option<serde_json::Value>,
    ) -> Self {
        Self {
            method,
            url,
            params,
            headers,
            body,
        }
    }

    pub async fn send(&self, args: &OverrideArgs) -> Result<ResponseExt> {
        let (headers, query, body) = self.generate(args)?;
        let client = Client::new();
        let req = client
            .request(self.method.clone(), self.url.clone())
            .headers(headers)
            .query(&query)
            .body(body)
            .build()?;
        let res = client.execute(req).await?;
        Ok(ResponseExt(res))
    }

    pub fn generate(&self, args: &OverrideArgs) -> Result<(HeaderMap, serde_json::Value, String)> {
        let mut headers = self.headers.clone();
        let mut query = self.params.clone().unwrap_or_else(|| json!({}));
        let mut body = self.body.clone().unwrap_or_else(|| json!({}));

        for (k, v) in &args.headers {
            headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
        }

        if !headers.contains_key(header::CONTENT_TYPE) {
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
        }

        for (k, v) in &args.query {
            query[k] = v.parse()?;
        }

        for (k, v) in &args.body {
            body[k] = v.parse()?;
        }

        let content_type = get_content_type(&headers);
        match content_type.as_deref() {
            Some("application/json") => {
                let body = serde_json::to_string(&body)?;
                Ok((headers, query, body))
            }
            Some("applications/x-www-form-urlencoded" | "multipart/form-data") => {
                let body = serde_urlencoded::to_string(&body)?;
                Ok((headers, query, body))
            }
            _ => Err(anyhow!("Unsupported content type")),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if let Some(ref params) = self.params {
            if !params.is_object() {
                return Err(anyhow!(
                    "Params must be an object but got:\n{}",
                    serde_yaml::to_string(params)?
                ));
            }
        }
        if let Some(ref body) = self.body {
            if !body.is_object() {
                return Err(anyhow!(
                    "Body must be an object but got:\n{}",
                    serde_yaml::to_string(body)?
                ));
            }
        }
        Ok(())
    }
}

pub fn get_content_type(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().unwrap().split(';').next().map(|s| s.to_string()))
}
