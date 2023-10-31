use proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // get the name of the annotated type
    let name = &ast.ident;

    // quote! macro lets use define the Rust code
    // that we want to return.
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! macro is a Rust built-in.
                // It converts a Rust expression and converts it
                // at compile-time into a string literal,
                // WITHOUT evaluating it.
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    // Convert the result of the macro's execution
    // into a TokenStream.
    gen.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as an 
    // abstract syntax tree (AST) that we can manipulate.
    // The type returned is `DeriveInput`.

    // The code in this outer function will be the same
    // virtually every time we write a procedural macro.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_hello_macro(&ast)
}