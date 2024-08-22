use singleton_attr::{derive, traits::Singleton};
use std::{thread, time};

#[derive(Debug, Default, derive::Singleton)]
pub struct Config {
    pub a: u32,
    pub b: u32,
    pub c: String,
}

fn main() {
    let config = Config::get_instance();
    config.a = 0;
    config.b = 0;
    config.c = "Hello, world!".to_string();

    // thread_1 sets a and b to random values every 5 seconds
    let thread_1 = thread::spawn(move || loop {
        let config = Config::get_instance();
        config.a = rand::random::<u32>();
        config.b = rand::random::<u32>();
        println!("THREAD_1 ---- a = {}, b = {}", config.a, config.b);
    });

    // thread_2 increments a by 1 every 2 seconds
    let thread_2 = thread::spawn(move || loop {
        let config = Config::get_instance();
        config.a += 1;
        println!("THREAD_2 ---- a += 1; a: {}", config.a);
    });

    // thread_3 prints the values of a and b every 1 second
    let thread_3 = thread::spawn(move || loop {
        let config = Config::get_instance();
        println!("THREAD_3 ---- a: {}, b: {}", config.a, config.b);
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();
    thread_3.join().unwrap();
}
