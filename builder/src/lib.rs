use syn::{parse_macro_input, DeriveInput};

use crate::{analyze::analyze, codegen::codegen, lower::lower};

mod analyze;
mod codegen;
mod lower;

pub(crate) type Ast = DeriveInput;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput) as Ast;
    let model = analyze(ast);
    let ir = lower(model);
    let rust = codegen(ir);
    rust.into()
}
