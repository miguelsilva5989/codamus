use std::collections::BTreeMap;

use super::RuntimeValue;

#[derive(Clone, Debug)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: BTreeMap<String, RuntimeValue>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Self {
        Self {
            parent,
            variables: BTreeMap::new(),
        }
    }

    pub fn resolve(&mut self, name: String) -> &mut Self {
        if self.variables.contains_key(&name) {
            return self;
        }

        if let Some(parent) = &mut self.parent {
            return parent.resolve(name);
        } else {
            panic!("Cannot resolve variable '{}' as it does not exist", name)
        }
    }

    pub fn lookup_var(&mut self, name: String) -> RuntimeValue {
        let env = self.resolve(name.clone());
        return env.variables.get(&name).unwrap().clone()
    }

    pub fn declare_var(&mut self, name: String, value: RuntimeValue) -> RuntimeValue {
        if self.variables.contains_key(&name) {
            panic!("Cannot declare variable '{}' as it is already defined", name)
        }

        self.variables.insert(name, value.clone());
        return value;
    }

    pub fn assign_var(&mut self, name: String, value: RuntimeValue) -> RuntimeValue {
        let env = self.resolve(name.clone());
        env.variables.entry(name).and_modify(|v| *v = value.clone());
        return value;
    }
}
