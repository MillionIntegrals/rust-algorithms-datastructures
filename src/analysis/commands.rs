use std::collections::HashMap;

pub trait CommandRunner {
    fn execute(&self);
}

struct FunctionRunner(fn() -> ());

impl CommandRunner for FunctionRunner {
    fn execute(&self) {
        (self.0)()
    }
}

pub struct CommandDescriptor {
    pub name: String,
    pub description: String,
    action: Box<dyn CommandRunner>,
}

impl CommandDescriptor {
    pub fn new(name: &str, description: &str, f: fn() -> ()) -> Self {
        CommandDescriptor {
            name: name.to_owned(),
            description: description.to_owned(),
            action: Box::new(FunctionRunner(f)),
        }
    }

    pub fn execute(&self) {
        self.action.execute()
    }
}

pub struct CommandMap {
    pub commands: HashMap<String, CommandDescriptor>,
}

impl CommandMap {
    pub fn new() -> Self {
        CommandMap {
            commands: HashMap::new(),
        }
    }

    pub fn add(&mut self, descriptor: CommandDescriptor) {
        self.commands.insert(descriptor.name.clone(), descriptor);
    }

    pub fn get(&self, name: &str) -> Option<&CommandDescriptor> {
        self.commands.get(name)
    }
}
