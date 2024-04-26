#![allow(dead_code)]

use std::cell::RefCell;

struct Foo;
impl Foo {
    fn do_something(&self) {
        println!("Do something");
    }
}

struct MyMutex<T> {
    flag: RefCell<bool>,
    data: T,
}

impl<T> MyMutex<T> {
    fn new(t: T) -> MyMutex<T> {
        MyMutex {
            flag: RefCell::new(false),
            data:t,
        }
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_work() {
        println!("test!!");
    }
}