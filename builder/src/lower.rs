use proc_macro2::{Ident, Span};
use syn::spanned::Spanned;
use syn::{Attribute, GenericArgument, Meta, Path, PathArguments, PathSegment, Type, Visibility};

use crate::analyze::Model;

pub struct Ir {
    pub target: Ident,
    pub builder: Ident,
    pub fields: Vec<FieldIr>,
}

pub struct FieldIr {
    pub name: Ident,
    pub ty: Type,
    pub required: bool,
    pub each: Option<String>,
    pub vis: Visibility,
    pub span: Span,
}

pub fn lower(model: Model) -> Ir {
    let mut fields = vec![];
    for f in model.fields.iter() {
        let optional_ty = extract_type_from_option(&f.ty);
        let (ty, required) = if let Some(ty) = optional_ty {
            (ty.clone(), false)
        } else {
            (f.ty.clone(), true)
        };

        for attr in &f.attrs {
            match attr.path.get_ident() {
                Some(ident) if ident == "builder" => {
                    if let Ok(meta) = attr.parse_meta() {
                        dbg!(meta);
                    }
                    // dbg!(attr.parse_args().unwrap());
                }
                _ => {}
            }
        }

        let ir = FieldIr {
            required,
            each: None,
            name: f.ident.clone().unwrap(),
            ty,
            vis: f.vis.clone(),
            span: f.span(),
        };
        fields.push(ir);
    }

    Ir {
        fields,
        target: model.target,
        builder: model.builder,
    }
}

fn extract_type_from_option(ty: &Type) -> Option<&Type> {
    fn extract_type_path(ty: &Type) -> Option<&Path> {
        match *ty {
            Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .into_iter()
            .find(|s| &idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(|path| extract_option_segment(path))
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}

#[cfg(test)]
mod test {
    use syn::{parse_quote, DeriveInput};

    use crate::analyze::analyze;
    use crate::lower::lower;

    #[test]
    fn test_parse() {
        let input: DeriveInput = parse_quote! {
            struct Target {
                pub executable: String,
                args: Vec<String>,
                cwd: Option<String>,
                env: Vec<String>,
            }
        };

        let model = analyze(input);

        let ir = lower(model);

        assert_eq!(ir.target.to_string(), "Target");
        assert_eq!(ir.builder.to_string(), "TargetBuilder");

        //assert_eq!(exe.vis, Visibility::Public);
        assert_eq!(ir.fields[0].required, true);

        // assert_eq!(args.vis, Visibility::Inherited);
        assert_eq!(ir.fields[1].required, true);

        // assert_eq!(cwd.vis, Visibility::Inherited);
        assert_eq!(ir.fields[2].required, false);

        // assert_eq!(env.vis, Visibility::Inherited);
        assert_eq!(ir.fields[3].required, true);
    }
}
