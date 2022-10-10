use anyhow::{anyhow, Result};
use askama::Template;
use heck::{AsSnakeCase, AsUpperCamelCase};
use litrs::Literal;
use proc_macro::TokenStream;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

pub fn get_token_literal(input: TokenStream) -> Result<String> {
    input
        .into_iter()
        .next()
        .and_then(|v| Literal::try_from(v).ok())
        .and_then(|v| match v {
            Literal::String(l) => Some(l.value().to_string()),
            _ => None,
        })
        .ok_or_else(|| anyhow!("Only string literal are allowed"))
}

#[derive(Template)]
#[template(path = "code.j2")]
pub struct StructsTemplate {
    st_list: Vec<St>,
}

impl StructsTemplate {
    pub fn new(st_list: Vec<St>) -> Self {
        Self { st_list }
    }

    pub fn try_new(filename: &str) -> Result<Self> {
        let content = fs::read_to_string(filename)?;
        let schema: Schema = serde_json::from_str(&content)?;
        Ok(Self::new(schema.to_st_list()))
    }

    pub fn render(filename: &str) -> Result<String> {
        let template = Self::try_new(filename)?;
        Ok(template.render()?)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Schema {
    title: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    properties: Option<HashMap<String, Schema>>,
}

#[derive(Debug)]
pub struct St {
    name: String,
    fields: Vec<Fd>,
}

#[derive(Debug)]
pub struct Fd {
    name: String,
    ty: String,
}

impl St {
    pub fn new(name: impl Into<String>, fields: Vec<Fd>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }
}

impl Fd {
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
        }
    }
}

impl Schema {
    pub fn to_st_list(&self) -> Vec<St> {
        let mut st_list = vec![];

        match self.ty.as_str() {
            "object" => {
                let fields: Vec<_> = self
                    .properties
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| process_ty(&mut st_list, k.as_str(), v))
                    .collect();

                st_list.push(St::new(
                    to_pascal_case(self.title.as_ref().unwrap()),
                    fields,
                ));
            }
            _ => panic!("Not implemented"),
        }
        st_list
    }
}

fn process_ty(st_list: &mut Vec<St>, key: &str, schema: &Schema) -> Fd {
    let name = to_snake_case(key);
    match schema.ty.as_str() {
        "object" => {
            let list = schema.to_st_list();
            st_list.extend(list);

            Fd::new(name, gen_type_name(schema.title.as_deref(), key))
        }
        "integer" => Fd::new(name, "i64"),
        "float" => Fd::new(name, "f64"),
        "string" => Fd::new(name, "String"),
        _ => panic!("Not implemented"),
    }
}

fn to_pascal_case(name: &str) -> String {
    AsUpperCamelCase(name).to_string()
}

fn to_snake_case(name: &str) -> String {
    AsSnakeCase(name).to_string()
}

fn gen_type_name(first: Option<&str>, second: &str) -> String {
    to_pascal_case(first.unwrap_or(second))
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;
    const PERSON1: &str = include_str!("../fixtures/person1.json");
    const PERSON2: &str = include_str!("../fixtures/person2.json");

    #[test]
    fn convert_simple_schema_to_st() {
        let schema: Schema = serde_json::from_str(PERSON1).unwrap();
        let st_list = schema.to_st_list();
        // println!("{:#?}", st_list);
        assert_eq!(st_list.len(), 1);
    }

    #[test]
    fn convert_nest_schema_to_st() {
        let schema: Schema = serde_json::from_str(PERSON2).unwrap();
        let st_list = schema.to_st_list();
        println!("{:#?}", st_list);
        assert_eq!(st_list.len(), 2);
    }

    #[test]
    fn render_structs() -> Result<()> {
        let result = StructsTemplate::render("fixtures/person2.json")?;
        println!("{:#?}", result);
        Ok(())
    }
}
