#![allow(dead_code)]

// 简单定义pub 无法做到向后兼容
#[derive(Debug)]
pub struct StudentInfo1 {
    pub name: String,
    pub age: u32,
    pub number: u32,
}

// 解决方案1
/**
 * 外部只能通过setter方法创建StudentInfo2 实例  不能通过常规创建对象的方式来创建
 * 而且对外解析时候 要在结构体字段上面加上 .. 
 */
#[non_exhaustive]
#[derive(Debug)]
pub struct StudentInfo2 {
    pub name: String,
    pub age: u32,
    pub number: u32,
}

impl StudentInfo2 {
    pub fn set_student_info(name: String, age: u32, number: u32) -> Self {
        StudentInfo2 { name, age, number }
    }
}



// 解决方案2
#[derive(Debug)]
pub struct StudentInfo3 {
    pub name: String,
    pub age: u32,
    pub number: u32,
    _b:(), // 添加一个私有成员  达到外面没法直接使用常规方法创建的目的
}