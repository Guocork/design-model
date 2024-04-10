#![allow(dead_code)]

use std::io;
use std::fs;
/**
 * 这里演示的是rust当中的动态分发 这个trait对象在编译时不知道他的具体类型
 * 多以是在运行时 才去看变量的类型 这个会在运行时有额外的开销
 * 与之相对的是 rust的单态化 即泛型 T 一般在编译时候就确定了变量的类型
 */
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_it_work() -> Result<(), Box<dyn std::error::Error>> {
        let arg = "-";

        let (mut stdin_read, mut file_read);

        let readable: &mut dyn io::Read = if arg == "-" {
            stdin_read = io::stdin();
            &mut stdin_read

        } else {
            file_read = fs::File::open(arg)?;
            &mut file_read
        };

        Ok(())
    }
}