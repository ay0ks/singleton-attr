use singleton_attr::{
    derive,
    traits::{SafeSingleton, Singleton},
};
use std::{thread, time};

#[derive(Debug, Default, derive::SafeSingleton)]
pub struct Config {
    pub a: u32,
    pub b: u32,
    pub c: String,
}

fn main() {
    {
        let mut config = Config::get_instance().unwrap();
        config.a = 0;
        config.b = 0;
        config.c = "Hello, world!".to_string();
        println!("MAIN ---- a: {}, b: {}", config.a, config.b);
    }

    // thread_1 sets a and b to random values
    let thread_1 = thread::spawn(move || loop {
        let mut config = Config::get_instance().unwrap();
        config.a = rand::random::<u32>();
        config.b = rand::random::<u32>();
        println!("THREAD_1 ---- a = {}, b = {}", config.a, config.b);
    });

    // thread_2 increments a by 1 every
    let thread_2 = thread::spawn(move || loop {
        let mut config = Config::get_instance().unwrap();
        config.a += 1;
        println!("THREAD_2 ---- a += 1; a: {}", config.a);
    });

    // thread_3 prints the values of a and b
    let thread_3 = thread::spawn(move || loop {
        let config = Config::get_instance().unwrap();
        println!("THREAD_3 ---- a: {}, b: {}", config.a, config.b);
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();
    thread_3.join().unwrap();
}
