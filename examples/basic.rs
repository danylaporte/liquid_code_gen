#![feature(proc_macro_hygiene)]

use liquid_code_gen::liquid;

fn main() {
    liquid!(
        r#"
        {% for i in (1..5) %}
            println!("test{{ i }}");
        {% endfor %}
    "#
    );
}
