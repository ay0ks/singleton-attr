# singleton-attr
Simple to use singleton procedural attribute

 - singleton-attr [![Crates.io Version](https://img.shields.io/crates/v/singleton-attr)](https://crates.io/crates/singleton-attr) [![docs.rs](https://img.shields.io/docsrs/singleton-attr)](https://docs.rs/singleton-attr/)
 - singleton-attr-proc-macro [![Crates.io Version](https://img.shields.io/crates/v/singleton-attr-proc-macro)](https://crates.io/crates/singleton-attr-proc-macro) [![docs.rs](https://img.shields.io/docsrs/singleton-attr-proc-macro)](https://docs.rs/singleton-attr-proc-macro/)

Example:
```rust
use singleton_attr::singleton;

#[singleton]
#[derive(Debug, Default)]
pub struct Config {
    pub a: i32,
    pub b: i32,
    pub c: String,
}

fn main() {
    let config_1 = Config::get_instance();

    println!("--BEFORE: {:#?}", config_1);
    {
        let mut config_1_lock = config_1.lock().unwrap();
        config_1_lock.a = 123;
        config_1_lock.b = -123;
        config_1_lock.c = "Hello, World!".to_string();
    }
    println!("--AFTER: {:#?}", config_1);

    let config_2 = Config::get_instance();
    println!("--BEFORE: {:#?}", config_2)
}
```
