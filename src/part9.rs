#![allow(dead_code)]

pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}
pub struct CreateTable;
impl Migration for CreateTable {
    fn execute(&self) -> &str {
        "create table"
    }
    fn rollback(&self) -> &str {
        "drop table"
    }
}

pub struct AddField;
impl Migration for AddField {
    fn execute(&self) -> &str {
        "add field"
    }
    fn rollback(&self) -> &str {
        "remove field"
    }
}

struct Scheme {
   commands: Vec<Box<dyn Migration>>, // 这里要用Box进行包裹 要不然 编译器在编译的过程中 没法确定变量类型
}

impl Scheme {
    fn new() -> Self {
        Scheme { commands: vec![] }
    }
    
    fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }

    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }

    fn rollback(&self) -> Vec<&str> {
        self.commands.iter().rev().map(|cmd| cmd.rollback()).collect()
    }
}

#[cfg(test)]
mod test {
   use super::*;

   #[test]
   fn test_it_work() {
      let mut schema = Scheme::new();

      let cmd = Box::new(CreateTable);
      schema.add_migration(cmd);
      let cmd = Box::new(AddField);
      schema.add_migration(cmd);

      assert_eq!(vec!["create table","add field"],schema.execute());
      assert_eq!(vec!["remove field","drop table"],schema.rollback());
   }
}
