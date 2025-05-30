use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn};

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        panic!("didn't expect extra attr argument");
    }

    let mut item = parse_macro_input!(item as ItemFn);
    let fn_name = item.sig.ident.to_string();

    
    let register = quote! {
        
    };
    
    quote!().into()
}
