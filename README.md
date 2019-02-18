[![Build Status](https://travis-ci.org/danylaporte/typed_fixedbitset.svg?branch=master)](https://travis-ci.org/danylaporte/typed_fixedbitset)

Proc macro that generate rust code from a liquid template.

## Example

```rust
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
```

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
[http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0) or the MIT license
[http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT), at your
option. This file may not be copied, modified, or distributed
except according to those terms.