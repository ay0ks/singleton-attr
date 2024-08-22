#[macro_use]
extern crate singleton_attr;

#[derive(Debug, Default)]
pub struct Config<T> {
    pub a: T,
    pub b: u32,
    pub c: String,
}

singleton_manual!([T:Default] Config<T>[T] where [T:Default] with [Box<String>]);
