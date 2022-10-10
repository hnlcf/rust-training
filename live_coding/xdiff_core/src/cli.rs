use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

/// Diff two http requests and compare the difference of the responses.
#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Clone, Subcommand)]
#[non_exhaustive]
pub enum Action {
    /// The default xdiff action.
    Run(RunArgs),
    /// Parse URLs to generate a profile.
    Parse,
}

#[derive(Debug, Clone, Parser)]
pub struct RunArgs {
    /// The config file name.
    #[clap(short, long, value_parser)]
    pub config: Option<String>,

    /// The profile name in config.
    #[clap(short, long, value_parser)]
    pub profile: String,

    /// The override args from cli input. Could be used to override the query, headers and body of the request.
    ///
    /// - For query params, use `-o key=value`.
    /// - For headers, use `-o %key=value`.
    /// - For body, use `-o @key=value`.
    #[clap(short, long, value_parser = parse_key_val, number_of_values=1)]
    pub override_args: Vec<KeyVal>,
}

#[derive(Debug, Clone)]
pub enum KeyValType {
    Query,
    Header,
    Body,
}

#[derive(Debug, Clone)]
pub struct KeyVal {
    pub key_type: KeyValType,
    pub key: String,
    pub value: String,
}

fn parse_key_val<'a>(s: &'a str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, '=');
    let retrieve = |v: Option<&'a str>| -> Result<&'a str> {
        Ok(v.ok_or_else(|| anyhow!("Invalid key value pair {}", s))?
            .trim())
    };

    let key = retrieve(parts.next())?;
    let value = retrieve(parts.next())?;

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('@') => (KeyValType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphabetic() => (KeyValType::Query, key),
        _ => return Err(anyhow!("Invalid key value pair")),
    };

    Ok(KeyVal {
        key_type,
        key: key.to_string(),
        value: value.to_string(),
    })
}

#[derive(Debug, Default)]
pub struct OverrideArgs {
    pub headers: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
}

impl From<Vec<KeyVal>> for OverrideArgs {
    fn from(args: Vec<KeyVal>) -> Self {
        let mut headers = vec![];
        let mut body = vec![];
        let mut query = vec![];

        for arg in args {
            match arg.key_type {
                KeyValType::Query => headers.push((arg.key, arg.value)),
                KeyValType::Header => body.push((arg.key, arg.value)),
                KeyValType::Body => query.push((arg.key, arg.value)),
            }
        }

        Self {
            headers,
            body,
            query,
        }
    }
}
