extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ Expr, TraitBoundModifier };


#[proc_macro_attribute]
pub fn aawait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let is_try = syn::parse::<TraitBoundModifier>(attr).expect("expect question mark");
    let expr: Expr = syn::parse(item).expect("expect expr");

    let expr = match is_try {
        TraitBoundModifier::None => quote! {
            #expr.await
        },
        TraitBoundModifier::Maybe(_) => quote! {
            #expr.await?
        }
    };

    expr.into()
}
