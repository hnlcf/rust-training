mod json_schema;

use crate::json_schema::get_token_literal;
use json_schema::StructsTemplate;
use proc_macro::TokenStream;

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let filename = get_token_literal(input).unwrap();
    let result = StructsTemplate::render(&filename).unwrap();
    result.parse().unwrap()
}
