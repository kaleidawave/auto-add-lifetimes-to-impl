use std::collections::HashSet;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{
    parse, punctuated::Punctuated, GenericParam, ItemImpl, Lifetime, LifetimeDef, Path,
    PathArguments, Type,
};

#[proc_macro_attribute]
pub fn auto_add_lifetimes_to_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Ok(mut item_impl) = parse::<ItemImpl>(item) {
        let mut constraints = HashSet::<GenericParam>::new();

        if let Some((_, ref r#trait, _)) = item_impl.trait_ {
            find_lifetimes_in_path(r#trait, &mut constraints);
        }

        find_lifetimes_in_type(&*item_impl.self_ty, &mut constraints);

        for constraint in constraints {
            item_impl.generics.params.push(constraint);
        }

        item_impl.into_token_stream().into()
    } else {
        quote!(compile_error!("Input was not a impl")).into()
    }
}

fn find_lifetimes_in_type(ty: &Type, constraints: &mut HashSet<GenericParam>) {
    match ty {
        Type::Path(ty_path) => {
            find_lifetimes_in_path(&ty_path.path, constraints);
        }
        Type::Verbatim(_ts) => {
            todo!()
        }
        Type::Group(group) => {
            find_lifetimes_in_type(&*group.elem, constraints);
        }
        _ => (),
    }
}

fn find_lifetimes_in_path(path: &Path, constraints: &mut HashSet<GenericParam>) {
    for segment in path.segments.iter() {
        if let PathArguments::AngleBracketed(generic_arguments) = &segment.arguments {
            for arg in generic_arguments.args.iter() {
                match arg {
                    syn::GenericArgument::Lifetime(lifetime) => {
                        constraints.insert(GenericParam::Lifetime(LifetimeDef {
                            attrs: vec![],
                            lifetime: Lifetime::new(
                                &format!("'{}", lifetime.ident),
                                Span::call_site(),
                            ),
                            colon_token: None,
                            bounds: Punctuated::new(),
                        }));
                    }
                    syn::GenericArgument::Type(ty) => {
                        find_lifetimes_in_type(ty, constraints);
                    }
                    syn::GenericArgument::Binding(binding) => {
                        find_lifetimes_in_type(&binding.ty, constraints);
                    }
                    syn::GenericArgument::Constraint(_) => todo!(),
                    syn::GenericArgument::Const(_) => {},
                }
            }
        }
    }
}
