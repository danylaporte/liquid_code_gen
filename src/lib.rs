//! A
//!
//! # Example
//! ```
//! #![feature(proc_macro_hygiene)]
//!
//! use liquid_code_gen::liquid;
//!
//! fn main() {
//!     liquid!(r#"
//!         {% for i in (1..5) %}
//!             println!("test{{ i }}");
//!         {% endfor %}
//!     "#);
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn liquid(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let s = unsafe { transform(&s) };

    s.parse().unwrap()
}

#[cfg(windows)]
#[allow(improper_ctypes)]
#[link(name = "liquidlib")]
extern "C" {
    fn transform(s: &str) -> String;
}
