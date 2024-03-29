mod builder;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::builder::BuilderContext;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ctxt = BuilderContext::new(input);
    ctxt.generate().into()
}
