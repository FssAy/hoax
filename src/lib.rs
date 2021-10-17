use quote::quote;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;


/// Macro that *"hides"* the string literal in the binary at compile time.
///
/// This procedural macro accept **only string literals** that starts and ends with `"`.
///
/// Panics only when the first token tree is not a string literal, any followings are ignored. <br>
/// Returns a `String` collected from a vector of chars.
///
///
/// ## Expansion
/// ```rust
/// hoax!("cat\n")
/// ```
/// is the same as
/// ```rust
/// {vec!['c', 'a', 't', '\n',].iter().collect::<String>()}
/// ```
///
/// ## Example
/// ```rust
/// #[macro_use] extern crate hoax;
///
/// println!("{}", "I am not hidden :c");
/// println!("{}", hoax!("I guess I am hidden c:"));
/// ```
///
///
#[proc_macro]
pub fn hoax(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into();
    let mut target = String::new();

    for tt in input2 {
        match tt {
            TokenTree::Literal(lit) => {
                let str = lit.to_string();

                if !(str.starts_with('"') && str.ends_with('"')) {
                    panic!("Only string literals can be passed!");
                }

                target = (&str[1..str.len()-1]).to_string();
                break;
            }
            _ => {
                panic!("Only string literals can be passed!");
            }
        }
    }

    let mut token_stream = quote!{};
    let mut is_breaking = false;
    for char in target.chars() {

        let char = match (is_breaking, char) {
            (breaks_neg, '\\') => {
                is_breaking = !is_breaking;

                if !breaks_neg {
                    continue;
                } else {
                    '\\'
                }
            },
            (false, char) => {
                char
            }
            (true, char) => {
                is_breaking = false;
                match char {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    def => def,
                }
            }
        };

        token_stream = quote!{#token_stream #char,};
    }

    token_stream = quote!{{vec![#token_stream].iter().collect::<String>()}};
    token_stream.into()
}
