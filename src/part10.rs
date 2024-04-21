#![allow(dead_code)]
type FnPtr = fn() -> String;

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
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_work() {
        
    }
}