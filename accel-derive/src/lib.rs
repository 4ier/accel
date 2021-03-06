#![recursion_limit = "128"]

mod builder;
mod host;
mod parser;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn kernel(_attr: TokenStream, func: TokenStream) -> TokenStream {
    let func: syn::ItemFn = syn::parse(func).expect("Not a function");
    let ptx_str = builder::compile_tokens(&func).expect("Failed to compile to PTX");
    host::func2caller(&ptx_str, &func).into()
}
