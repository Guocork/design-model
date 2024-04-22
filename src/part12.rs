#![allow(dead_code)]
/**
 * 解释器模式
 */
pub struct Interpreter<'a> {
    it: std::str::Chars<'a>  // 这是一个字符迭代器  相当于 let s = "hello";  let chars_iter = s.chars(); 获取字符串的字符迭代器
}

impl<'a> Interpreter<'a> {
    pub fn new(infix: &'a str) -> Self {
        Self { it: infix.chars() }
    }

    fn next_char(&mut self) -> Option<char> {
        self.it.next()
    }

    pub fn interpret(&mut self, out: &mut String) {
        self.term(out);

        while let Some(op) = self.next_char() {  // 这里是想吧一个式子转化成为逆波兰式 即符号在表达式的末尾
            if op == '+' || op == '-' {
                self.term(out);
                out.push(op);
            } else {
                panic!("Unexpected symbol '{}'",op);
            }
        }
    }

    fn term(&mut self, out: &mut String) {
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => out.push(ch),
            Some(ch) => panic!("Unexpected symbol '{}'",ch),
            None => panic!("Unexpected end of string"),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_work() {
        let mut intr = Interpreter::new("2+3");
        let mut postfix = String::new();
        intr.interpret(&mut postfix);
        assert_eq!(postfix,"23+");

        intr = Interpreter::new("1-2+3-4");
        postfix.clear();
        intr.interpret(&mut postfix);
        assert_eq!(postfix,"12-3+4-");
    }
}