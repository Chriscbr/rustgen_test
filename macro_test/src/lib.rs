extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(TraceMacro)]
pub fn trace_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_trace_macro(&ast)
}

fn impl_trace_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            fn execute_code(input: &str) {
                println!("Hello, Macro! This struct is {}, and my name is {}.", stringify!(#name), input);
            }
        }
    };
    gen.into()
}
