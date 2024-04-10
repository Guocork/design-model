#![allow(dead_code)]
use std::rc::Rc;
/**
 * 这里主要是 要使得定义的变量 要尽量的在函数业务处理范围内
 * 这样代码对外界影响是最小的
 * 详见下面的这两段相同效果的代码 后面定义的两个新的变量的生命周期是不一样的
 * 第一个生命周期更短 对整体代码的影响最小
 * 编码习惯： 最好定义的变量在相对对应的函数代码块内
 */
#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_it_work() {
        // 好的操作
        let a = Rc::new(1);
        let b = Rc::new(2);
        let c = Rc::new(3);

        let closure = {
            let b1 = b.clone();
            let c1 = c.as_ref();
            move || {
                let ret = *a + *b1 + *c1;
                println!("ret = {:?}",ret);
            }
        };

        closure();

        println!("+++++++++++++++++++++++++++++++");
        // 差的操作
        let a = Rc::new(1);
        let b = Rc::new(2);
        let c = Rc::new(3);

        let b_cloned = b.clone();
        let c_borrowed = c.as_ref();

        let closure1 = move || {
            let ret = *a + *b_cloned + *c_borrowed;
            println!("ret = {:?}",ret);
        };

        closure1();


    }
}