use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};

use crate::lower::{FieldIr, Ir};

pub type Rust = proc_macro::TokenStream;

pub fn codegen(ir: Ir) -> Rust {
    let Ir {
        target,
        builder,
        fields,
    } = ir;

    let builder_fields = fields.iter().map(to_builder_field);
    let builder_init = fields.iter().map(to_builder_init);
    let setters = fields.iter().map(to_setter).flatten();
    let build_calls = fields.iter().map(to_build_call);

    let expanded = quote! {
        use std::error::Error;

        pub struct #builder {
            #( #builder_fields ),*
        }

        impl #builder {

            #( #setters )*

            pub fn build(&mut self) -> Result<#target, Box<dyn Error>> {
                Ok(#target {
                    #( #build_calls ),*
                })
            }
        }

        impl #target {
            pub fn builder() -> #builder {
                #builder {
                    #( #builder_init ),*
                }
            }
        }

    };

    expanded.into()
}

fn to_builder_field(f: &FieldIr) -> TokenStream {
    let FieldIr { ty, name, span, .. } = f;
    quote_spanned!(*span=> #name: Option<#ty>)
}

fn to_builder_init(f: &FieldIr) -> TokenStream {
    let FieldIr { name, span, .. } = f;
    quote_spanned!(*span=> #name: None)
}

fn to_setter(f: &FieldIr) -> Vec<TokenStream> {
    let FieldIr {
        name,
        ty,
        span,
        each,
        ..
    } = f;

    let mut items = vec![];

    if let Some(each) = each {
        let each = format_ident!("{}", each);
        let each_item = quote_spanned! {*span=>
            pub fn #each(&mut self, #each: #ty) -> &mut Self {
                if let Some(mut v) = self.#name {
                    v.push(#each);
                } else {
                    self.#name = Some(vec![#each]);
                }
                self
            }
        };

        items.push(each_item);
    }

    if each.is_none() {
        let default_item = quote_spanned! {*span=>
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        };
        items.push(default_item);
    } else if each != &Some(name.to_string()) {
        let default_item = quote_spanned! {*span=>
            pub fn #name(&mut self, #name: Vec<#ty>) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        };
        items.push(default_item);
    }
    items
}

fn to_build_call(f: &FieldIr) -> TokenStream {
    let FieldIr {
        name,
        span,
        required,
        ..
    } = f;

    if *required {
        let msg = format!("missing value for field '{}'", name.clone());
        quote_spanned! {*span=>
            #name: self.#name.take().ok_or(Box::<dyn Error>::from(#msg))?
        }
    } else {
        quote_spanned! {*span=>
            #name: self.#name.clone()
        }
    }
}
