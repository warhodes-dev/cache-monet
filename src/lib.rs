#![feature(let_chains)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{self, parse_quote};
use darling::FromMeta;
use darling::ast::NestedMeta;

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    size: Option<usize>,
}

#[derive(Debug)]
enum CacheType {
    LruCache,
    HashMap,
}

#[derive(Debug)]
struct Config {
    size: Option<usize>,
    cache_type: CacheType,
}

impl Config {
    fn from(args: &MacroArgs) -> Result<Self, Box<dyn std::error::Error>> {
        let cache_type = match args.size {
            Some(0) => return Err("Cannot create cache of size 0.".into()),
            Some(_) => CacheType::LruCache,
            None => CacheType::HashMap,
        };
        Ok( Config { 
            size: args.size, 
            cache_type 
        })
    }
}

#[proc_macro_attribute]
pub fn cached(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(item)
        .expect("Failed to parse TokenStream. Check that the return type is correct.");

    let macro_args_list = NestedMeta::parse_meta_list(args.into())
        .expect("Failed to parse macro arguments");

    let macro_args = MacroArgs::from_list(&macro_args_list)
        .expect("Failed to build darling structure from macro arguments");

    let config = Config::from(&macro_args)
        .expect("Failed to build config from macro arguments");

    if cfg!(debug_assertions) {
        println!("Cache Config:\n{config:#?}");
    }

    impl_cached(&ast, &config)
}

fn impl_cached(func: &syn::ItemFn, config: &Config) -> TokenStream {
    let (param_ids, param_types): (Vec<_>, Vec<_>) = func.sig.inputs.iter()
        .map(|argument| {
            if let syn::FnArg::Typed(parameter) = argument
            && let syn::Pat::Ident(parameter_id) = &*parameter.pat {
                let param_type = &*parameter.ty;
                let param_id = &parameter_id.ident;
                (param_id, param_type)
            } else { 
                unimplemented!("Unsupported argument form."); 
            }
        }).unzip();

    let key_type: syn::Type = parse_quote!((#(#param_types),*));

    let syn::ReturnType::Type(_, value_type) = &func.sig.output 
    else {
        panic!("Should not memoize function with no return type");
    };

    let func_sig = &func.sig;
    let func_body = &func.block;
    let cache_id = format_ident!("{}_CACHE", &func.sig.ident.to_string().to_uppercase());

    let (cache_type, cache_init, cache_insert_method) = match config.cache_type {
        CacheType::LruCache => {
            if let Some(size) = config.size {
                (  
                    quote! { lru::LruCache<#key_type, #value_type> },
                    quote! { lru::LruCache::new(std::num::NonZeroUsize::new(#size).unwrap()) },
                    quote! { put },

                )
            } else {
                panic!("Can't have LRU Cache of size 0")
            }
        },
        CacheType::HashMap => {
            (
                quote! { std::collections::HashMap<#key_type, #value_type> },
                quote! { std::collections::HashMap::new() },
                quote! { insert },
            )
        }
    }; 

    let memoized_func = quote! {
        thread_local! {
            static #cache_id: std::cell::RefCell<#cache_type> =
                std::cell::RefCell::new(#cache_init)
        }

        #func_sig {
            // Note, nightly <cell>.with_borrow_mut() cannot be used *or enabled* here because we are 
            // in the macro expansion. The end user would have to enable this with #![feature(...)]
            let key = (#(#param_ids),*);
            if let Some(value) = #cache_id.with(|cache| cache.borrow_mut().get(&key).cloned() ) {
                value
            } else {
                let value = (||#func_body)();
                #cache_id.with(|cache| {
                    cache.borrow_mut().#cache_insert_method(key, value.clone());
                });
                value
            }
        }
    };

    memoized_func.into()
}