# singleton-attr
Simple to use singleton procedural attribute.

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
You can run it with `cargo run --example example`
