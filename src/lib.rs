extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn cached(
    _attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(item)
        .expect("Failed to parse TokenStream. Check that the return type is correct.");
    impl_prepend_task(&ast)
}

fn impl_prepend_task(ast: &syn::ItemFn) -> TokenStream {
    let func_name = &ast.sig.ident;
    let func_body = &ast.block;
    let gen = quote! {
        fn #func_name() {
            println!("This occurs before the function is run");
            #func_body
            println!("This occurs after the function is run");
        }
    };
    gen.into()
}