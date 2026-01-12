use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, Ident, ItemFn, Type, parse_macro_input};

/// Determines if a given `arg` is of type `Args` (from the parent crate's `utils::arg`).
fn is_args(arg: &FnArg) -> bool {
    let FnArg::Typed(arg) = arg else { return false };
    let Type::Path(type_path) = arg.ty.as_ref() else {
        return false;
    };
    // since `Args` requires an explicit elided lifetime (<'_>), we have to match for `type_path`'s first segment (`Args`<'_>)
    let Some(segment) = type_path.path.segments.first() else {
        return false;
    };

    segment.ident == "Args"
}

/// Using this macro, we can add a command to a global registry so we can find them without manual work.
///
/// # Panics
///
/// Will panic if any attribute argument is put into this macro.
#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    assert!(attr.is_empty(), "didn't expect extra attr argument");

    let item = parse_macro_input!(item as ItemFn);
    let fn_ident = item.sig.ident.clone();
    let fn_name = item.sig.ident.to_string();
    let has_args = item.sig.inputs.iter().any(is_args);

    // Create a unique wrapper function name
    let wrapper_ident = Ident::new(&format!("__boxed_{fn_name}"), fn_ident.span());

    let output = if has_args {
        quote! {
            fn #wrapper_ident(ctx: &::serenity::all::Context, replyer: &::utils::reply::Replyer<'_>, args: ::utils::Args<'_>) -> ::glue::RunResult {
                let fut = async move {
                    self::#fn_ident(ctx, replyer, args).await
                };
                Box::new(fut)
            }

            ::inventory::submit! {
                ::glue::BotCommand::new(#fn_name, #has_args, self::register, None, Some(#wrapper_ident))
            }

            #item
        }
    } else {
        quote! {
            fn #wrapper_ident(ctx: &::serenity::all::Context, replyer: &::utils::reply::Replyer<'_>) -> ::glue::RunResult {
                let fut = async move {
                    self::#fn_ident(ctx, replyer).await
                };
                Box::new(fut)
            }

            ::inventory::submit! {
                ::glue::BotCommand::new(#fn_name, #has_args, self::register, Some(#wrapper_ident), None)
            }

            #item
        }
    };

    output.into()
}
