#[macro_use]
extern crate singleton_attr;
use singleton_attr::traits::Singleton;

#[derive(Debug, Default)]
pub struct Config {
    pub a: i32,
    pub b: &'static str,
    pub c: String,
}

singleton_manual!(Config);

fn main() {
    Config::init_instance(Config {
        a: 42,
        b: "hello",
        c: "world".to_string(),
    });

    let config = Config::get_instance();

    println!("{:?}", config);
}
