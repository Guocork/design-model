#![allow(dead_code)]
/**
 * 新类型 零成本抽象
 */


#[derive(Debug)]
struct StudentInfo {
    name: &'static str,
    id: &'static str,
    number: &'static str,
}

impl StudentInfo {
    fn new(name: &'static str,id: &'static str,number: &'static str) -> Self {
        StudentInfo {
            name,
            id,
            number:number,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn id(&self) -> &'static str {
        self.id
    }
    
    fn number(&self) -> &'static str {
        self.number
    }
}

struct PersonInfo(StudentInfo); // 没有实现打印的宏 自定义了暴露的方法 封装时候进行控制 增加或者减少功能

impl PersonInfo {
    fn new(s: StudentInfo) -> Self {
        PersonInfo(s)
    }

    fn name(&self) -> &'static str {
        self.0.name()
    }

    fn id(&self) -> &'static str {
        self.0.id()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_work() {
        
    }
}