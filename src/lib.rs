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
use liquid::filters::invalid_argument_count;
use liquid::compiler::FnFilterValue;
use liquid::value::error::Error;
use liquid::value::Value;
use proc_macro::TokenStream;

#[proc_macro]
pub fn liquid(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let s = s[3..s.len() - 2]
        .replace("\\\"", "\\x22")
        .replace("\\#", "#");

    let parser = liquid::ParserBuilder::with_liquid()
        .filter("snake", snake as FnFilterValue)
        .build()
        .unwrap()
        .parse(&s)
        .unwrap();

    let globals = liquid::value::Object::new();
    let out = parser.render(&globals).unwrap();

    out.parse().unwrap()
}

fn snake(input: &Value, args: &[Value]) -> Result<Value, Error> {
    check_args_len(args, 0, 0)?;

    let s = input.to_str();
    Ok(Value::scalar(s.to_snake_case()))
}

fn check_args_len(args: &[Value], required: usize, optional: usize) -> Result<(), Error> {
    if args.len() < required {
        return Err(invalid_argument_count(format!(
            "expected at least {}, {} given",
            required,
            args.len()
        )));
    }
    if required + optional < args.len() {
        return Err(invalid_argument_count(format!(
            "expected at most {}, {} given",
            required + optional,
            args.len()
        )));
    }
    Ok(())
}
