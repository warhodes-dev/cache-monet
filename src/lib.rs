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
    let func_sig = &ast.sig;
    let func_body = &ast.block;
    let gen = quote! {
        #func_sig {
            println!("This occurs before the function is run");
            #func_body
            // Be careful inserting code here. Early implicit returns might get suppressed.
            // Perhaps you can replace early implicit returns with explicit returns?
        }
    };
    gen.into()
}