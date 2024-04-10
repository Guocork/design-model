#![allow(dead_code)]
/**
 * 代码习惯 利用了 rust 的 shadow 属性 所有权的重新赋值
 */
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_work() {
        let mut data = vec![2, 1, 4, 10, 3, 5];
        data.sort();
        let data = data; // 对data进行重新绑定，data变成不可变变量

        println!("{:?}",data[2]);


        // =====================================================
        // 使用嵌套快 与上面等价
        let data = {
        let mut data = vec![2, 1, 4, 10, 3, 5];
        data.sort();
        data
        };

        println!("{:?}",data[2]);

    }
}