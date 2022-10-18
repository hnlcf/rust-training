use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::iter::Map;
use syn::{
    punctuated::{Iter, Punctuated},
    token::Comma,
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Path, PathArguments, PathSegment,
    Type, TypePath,
};

type TokenStreamIter<'a> = Map<Iter<'a, Field>, fn(&Field) -> TokenStream>;

pub struct BuilderContext {
    name: Ident,
    fields: Punctuated<Field, Comma>,
}

impl BuilderContext {
    pub fn new(input: DeriveInput) -> Self {
        let name = input.ident;
        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("Unsupported data type");
        };

        Self { name, fields }
    }

    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        // the builder name, e.g. Command -> CommandBuilder
        let builder_name = Ident::new(&format!("{}Builder", name), name.span());
        // the optional fields, e.g. executable: String -> executable: Option<String>
        let optionized_fileds = self.gen_optionize_fields();
        let methods = self.gen_methods();
        let assigns = self.gen_assigns();

        quote! {
            #[derive(Debug, Default)]
            pub struct #builder_name{
                #(#optionized_fileds,)*
            }

            impl #builder_name {
                #(#methods)*

                pub fn finish(mut self) -> Result<#name, &'static str> {
                    Ok(
                        #name {
                            #(#assigns,)*
                        }
                    )
                }
            }

            impl #name {
                pub fn builder() -> #builder_name {
                    Default::default()
                }
            }
        }
    }

    fn gen_optionize_fields(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            let (_, ty) = try_get_option_inner(&f.ty);
            let name = &f.ident;
            quote! {
                #name: std::option::Option<#ty>
            }
        })
    }

    fn gen_methods(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            let (_, ty) = try_get_option_inner(&f.ty);
            let name = &f.ident;
            quote! {
                pub fn #name(mut self, v: impl Into<#ty>) -> Self {
                    self.#name = Some(v.into());
                    self
                }
            }
        })
    }

    fn gen_assigns(&self) -> TokenStreamIter {
        self.fields.iter().map(|f| {
            let name = &f.ident;
            let (optional, _) = try_get_option_inner(&f.ty);
            if optional {
                quote! {
                    #name: self.#name.take()
                }
            } else {
                quote! {
                    #name: self.#name.take().ok_or(concat!(stringify!(#name), " needs to be set!"))?
                }
            }
        })
    }
}

fn try_get_option_inner(ty: &Type) -> (bool, &Type) {
    if let Some(s) = get_path_segments(ty) {
        if s.ident == "Option" {
            let ty = match &s.arguments {
                PathArguments::AngleBracketed(angle) => match angle.args.iter().next() {
                    Some(syn::GenericArgument::Type(t)) => t,
                    _ => panic!("Unsupported other generic types"),
                },
                _ => panic!("Unsupported other path arguments"),
            };
            return (true, ty);
        }
    }
    (false, ty)
}

fn get_path_segments(ty: &Type) -> Option<&PathSegment> {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        return segments.iter().next();
    }
    None
}
