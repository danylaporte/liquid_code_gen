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

use inflector::Inflector;
use liquid::compiler::Filter;
use liquid::error::Result;
use liquid::interpreter::Context;
use liquid::value::Value;
use liquid_derive::*;
use proc_macro::TokenStream;

#[proc_macro]
pub fn liquid(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let s = s[3..s.len() - 2]
        .replace("\\\"", "\\x22")
        .replace("\\#", "#");

    let parser = liquid::ParserBuilder::with_liquid()
        .filter(Snake)
        .build()
        .unwrap()
        .parse(&s)
        .unwrap();

    let globals = liquid::value::Object::new();
    let out = parser.render(&globals).unwrap();

    out.parse().unwrap()
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "snake",
    description = "Transform a string into snake case",
    parsed(SnakeFilter)
)]
struct Snake;

#[derive(Debug, Default, Display_filter)]
#[name = "snake"]
struct SnakeFilter;

impl Filter for SnakeFilter {
    fn evaluate(&self, input: &Value, _context: &Context) -> Result<Value> {
        let s = input.to_str();
        Ok(Value::scalar(s.to_snake_case()))
    }
}
