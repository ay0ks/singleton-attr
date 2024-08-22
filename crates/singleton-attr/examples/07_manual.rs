#[macro_use]
extern crate singleton_attr;

#[derive(Debug, Default)]
pub struct Config<T> {
    pub a: T,
    pub b: i32,
    pub c: String,
}

singleton_manual!(Config<T> with T = i32 where T: Default);
singleton_safe_manual!(Config<T> with T = i32 where T: Default);

fn main() {}
