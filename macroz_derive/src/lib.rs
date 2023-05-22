extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(TypeName)]
pub fn print_typename_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_print_typename(&ast)
}

fn impl_print_typename(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TypeName for #name {
            fn print_typename() {
                println!(
                    "My typename is {}!",
                    stringify!(#name)
                );
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn prepend_task(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(item).expect("Yep, it happened");
    impl_prepend_task(&ast)
}

fn impl_prepend_task(ast: &syn::ItemFn) -> TokenStream {
    let func_name = &ast.sig.ident;
    let func_body = &ast.block;
    let gen = quote! {
        fn #func_name() {
            println!("This prints before");
            #func_body
        }
    };
    gen.into()
}