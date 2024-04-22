#![allow(dead_code)]
type Migration<'a> = Box<dyn Fn() -> &'a str>;

struct Scheme<'a> {
    executes: Vec<Migration<'a>>,
    rollbacks: Vec<Migration<'a>>,
}

impl<'a> Scheme<'a> {
    fn new() -> Self {
        Self {
            executes:vec![],
            rollbacks: vec![],
        }
    }

    fn add_migration<E,R>(&mut self, execute: E,rollback:R)
    where
        E:Fn() -> &'a str + 'static,
        R:Fn() -> &'a str + 'static, 
    {
        self.executes.push(Box::new(execute));
        self.rollbacks.push(Box::new(rollback));   
    }

    fn execute(&self) -> Vec<&str> {
        self.executes.iter().map(|cmd| cmd()).collect()
    }

    fn rollback(&self) -> Vec<&str> {
        self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
    }
}

fn add_field() -> &'static str {
    "add field"
}

fn remove_field() -> &'static str {
    "remove field"
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_work() {
        let mut scheme = Scheme::new();
        scheme.add_migration(|| "create table", || "drop table");
        scheme.add_migration(add_field, remove_field);
        assert_eq!(vec!["create table","add field"], scheme.execute());
        assert_eq!(vec!["remove field","drop table"], scheme.rollback());
    }
}