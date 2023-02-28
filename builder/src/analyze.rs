use proc_macro2::Ident;
use quote::format_ident;
use syn::{Data, Field, Fields};

use crate::Ast;

pub struct Model {
    pub target: Ident,
    pub builder: Ident,
    pub fields: Vec<Field>,
}

pub fn analyze(ast: Ast) -> Model {
    let item = ast;

    let mut parsed: Vec<Field> = vec![];

    let target = item.ident.clone();
    let builder = format_ident!("{}Builder", target);

    if let Data::Struct(ref data) = item.data {
        if let Fields::Named(ref fields) = data.fields {
            for f in fields.named.iter() {
                parsed.push(f.clone());
            }
        }
    }
    Model {
        target,
        builder,
        fields: parsed,
    }
}
