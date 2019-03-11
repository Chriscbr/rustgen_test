extern crate proc_macro;

use macro_test::TraceMacro;
use quote::quote;
use proc_macro2::{TokenStream, TokenTree};
use syntax::{parse, print::pprust, source_map};

#[derive(TraceMacro)]
struct Trace;

// #[derive(TraceMacro)] expands during compile-time to give
// the Trace struct an execute_code function.
// the TraceMacro macro is defined in its own crate named macro_test
fn ex1() {
    let input = "bob";
    Trace::execute_code(input);
    // prints "Hello Macro! This struct is Trace, and my name is bob."
}

fn ex2() {
    // here is some code I would like to export
    let stmt: TokenStream = quote! {
        let x: i32 = 10;
        foo();
        bar();
    };
    let stmt_clone: TokenStream = stmt.clone();

    let stmt_str: String = stmt
        .into_iter()
        .map(|token| token.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", stmt_str);
    // prints "let x : i32 = 10 ; foo () ; bar () ;"

    let stmt_tokens: Vec<TokenTree> = stmt_clone
        .into_iter()
        .collect();
    println!("{:?}", stmt_tokens);
    // prints out list of tokens
}

fn ex3() {
    // // unfortunately this panics at runtime
    // let stmt: String = String::from("3 + 5");
    // let parser_sess = parse::ParseSess::new(source_map::FilePathMapping::empty());
    // let file = source_map::FileName::Custom(String::from("foo"));
    // let mut parser = parse::new_parser_from_source_str(&parser_sess, file, stmt);
    // let tokens = parser.parse_tokens();
    // println!("{}", pprust::tokens_to_string(tokens));
}

fn main() {
    ex1();
    ex2();
    ex3();
}
