#![allow(dead_code)]

/**
 * 1. push 和 extend都是向集合中添加元素  
 * push 一次只能向集合中添加一个元素 extend 传入一个遍历器 一次可以放进去多个元素
 * extend 会比 push 更加高效 
 * 这里 Option枚举中实现了遍历器trait 可以传入 extend 方法 
 * Option 可以直接把一个变量当成里面得元素来用
 */
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_it_work() {
        let a = Some("a string");
        let mut s1 = vec!["a","b","c"];
        s1.extend(a);
        println!("s1:{:?}",s1);

        println!("++++++++++++++++++++++++++++++++");

        let b = Some("b string");
        let s1 = vec!["d","e","f"];

        for s in s1.iter().chain(b.iter()) {
            println!("item: {}",s);
        }
    }
}