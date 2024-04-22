#![allow(dead_code)]

mod part2;
mod part3;
mod part4;
mod part5;
mod part6;
mod part7;
mod part8;
mod part9;
mod part10;
mod part11;
mod part12;
fn say_hello1(name: &str) -> String {
    format!("Hello {} !",name)
}

fn say_hello2(name: &str) -> String {
    let mut result = "Hello".to_owned();
    result.push_str(name);
    result.push('!');
    result
}
/* 
上面两个函数等价 都是进行字符串以及字符串字面量的拼接 
推荐使用第一种方法 写的代码少
*/

fn main() {
    println!("Hello, world!");
}


