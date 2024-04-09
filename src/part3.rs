#![allow(dead_code)]
/* 
    在结构体或者枚举中取值的 涉及到原始结构体字段类型所有权的移动
    推荐使用std::mem::replace() 或者 std::mem::take()方法
    这两种方式不会clone原始值
*/
use std::mem;
#[derive(Debug)]
enum MyEnum {
    A {name: String, x: u8 },
    B {name: String },
}

fn convert_a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x:0 } = e {
        *e = MyEnum::B { 
            name: mem::take(name),  
        }
    }
}

fn convert_a_to_b2(e: &mut MyEnum) {
    if let MyEnum::A { name, x:0 } = e {
        *e = MyEnum::B { 
            name: mem::replace(name,String::new()),  
        }
    }
}

fn convert_a_to_b3(e: MyEnum) -> MyEnum {
    if let MyEnum::A { name, x:0 } = e {
        return MyEnum::B { name: name.clone() }
    } else {
        return e
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_work() {
        let mut a = MyEnum::A { 
            name: "A".to_string(), 
            x: 0
        };
        println!("Before Convert,a is {:?}",a);

        convert_a_to_b(&mut a);
        println!("After convert1,a is {:?}",a);

        convert_a_to_b2(&mut a);
        println!("After convert2,a is {:?}",a);
    }
}