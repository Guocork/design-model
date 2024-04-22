#![allow(dead_code)]
type FnPtr = fn() -> String;  // 函数指针是一个类型

struct Command {
    execute: FnPtr,
    rollback:FnPtr,
}

struct Scheme {
    commands: Vec<Command>,
}

impl Scheme {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_migration(&mut self,execute: FnPtr,rollback: FnPtr) {
        self.commands.push(Command {execute,rollback});
    }

    fn execute(&self) -> Vec<String> {
        self.commands.iter().map(|cmd| (cmd.execute)()).collect()
    }

    fn rollback(&self) -> Vec<String> {
        self.commands.iter().rev().map(|cmd|(cmd.rollback)()).collect()
    }
}

fn add_field() -> String {
    "add field".to_string()
}

fn remove_field() -> String {
    "remove field".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_work() {
        let mut scheme = Scheme::new();
        scheme.add_migration(|| "create table".to_string(), || "drop table".to_string());
        scheme.add_migration(add_field, remove_field);
        assert_eq!(vec!["create table","add field"], scheme.execute());
        assert_eq!(vec!["remove field","drop table"], scheme.rollback());
    }
}