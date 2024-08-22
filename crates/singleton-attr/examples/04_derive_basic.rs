use singleton_attr::{
    derive,
    traits::{SafeSingleton, Singleton},
};
use std::{thread, time};

#[derive(Debug, Default, derive::Singleton)]
pub struct Config {
    pub a: i32,
    pub b: i32,
    pub c: String,
}

fn main() {
    let config_1 = Config::get_instance();

    println!("--BEFORE: {:#?}", config_1);
    config_1.a = 123;
    config_1.b = -123;
    config_1.c = "Hello, World!".to_string();
    println!("--AFTER: {:#?}", config_1);

    let config_2 = Config::get_instance();
    println!("--BEFORE: {:#?}", config_2)
}
