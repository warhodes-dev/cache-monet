extern crate proc_macro;

use proc_macro::TokenStream;
use quote::format_ident;
use syn::{self, spanned::Spanned};

#[proc_macro_attribute]
pub fn cached(
    _attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(item)
        .expect("Failed to parse TokenStream. Check that the return type is correct.");

    impl_cached(&ast)
}

fn impl_cached(func: &syn::ItemFn) -> TokenStream {
    let fn_name = &func.sig.ident;
    let fn_sig = &func.sig;
    let fn_body = &func.block;

    let memo_fn_name = quote::format_ident!("memoized_{}", fn_name);
    let flush_name = syn::Ident::new(&format!("memoized_flush_{}", fn_name), fn_sig.span());
    let map_name = format_ident!("memoized_mapping_{}", fn_name);

    let inputs = get_input_pairs(&func.sig.inputs);

    let gen = quote::quote! {
        #fn_sig {
            println!("This occurs before the function is run");
            #fn_body
            // Be careful inserting code here. Early implicit returns might get suppressed.
            // Perhaps you can replace early implicit returns with explicit returns?
        }
    };
    gen.into()
}

fn get_input_pairs(
    inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>
) -> Vec<(syn::Ident, syn::Type)> {
    inputs
        .iter()
        .map(|input| match input {
            syn::FnArg::Receiver(_) => panic!("methods (functions taking 'self') are not supported"),
            syn::FnArg::Typed(pat_type) => (*match_pattern_type(&pat_type),*pat_type.ty.clone()),
        })
        .collect()
}

fn match_pattern_type(pat_type: &&syn::PatType) -> Box<syn::Ident> {
    match &std::ops::Deref::deref(&pat_type.pat) {
        syn::Pat::Ident(pat_ident) => {
            if pat_ident.mutability.is_some() {
                let mut pat_ident_is_mut = pat_ident.clone();
                pat_ident_is_mut.mutability = None;
                Box::new(pat_ident_is_mut.ident.clone())
            } else {
                Box::new(pat_ident.ident.clone())
            }
        }
        _ => panic!("Patterns as input args not supported")
    }
}
