#![allow(dead_code)]

use std::ops::Deref;
/*
实现Default 宏的前提是 该结构体中的每一个字段都实现了Default宏
*/
#[derive(Default)]  
pub struct Person {
    name: String,
    age: u32
}

// impl Default for Person {
//     fn default() -> Self {
//         Self {
//             name: "".to_string(),
//             age: 0
//         }
//     }
// }

fn test() {
    let a = vec![1,2,3];
    let b = Box::new(a.deref());
}


/* 类似析构函数 */
    #[derive(Debug)]
    struct A(u8);
    impl Drop for A {
        fn drop(&mut self) {
            println!("A exit!");
            // panic!("in drop");
            println!("A exit2");   // Drop trait 中的panic后的函数不会执行
        }
    }
    
    #[derive(Debug)]
    struct B(u8);
    impl Drop for B {
        fn drop(&mut self) {
            println!("B exit!")
        }
    }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let a = A(1);
        {
            let b = B(2);
        }
        panic!("error!"); // 这里的panic！不会影响Drop trait 中定义函数的执行
    }
}