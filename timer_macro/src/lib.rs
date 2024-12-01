extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as a function
    let input = parse_macro_input!(item as ItemFn);

    // Extract the function's signature and body
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    // Generate the wrapped function
    let output = quote! {
        #(#attrs)*
        #vis #sig {
            let start = std::time::Instant::now();
            let result = (|| #block)();
            println!("Duration: {:.2?}", start.elapsed());
            result
        }
    };

    output.into()
}
