use super::*;
use alloc::collections::VecDeque;
use alloc::string::String;

#[derive(Debug, Clone)]
pub enum Symbol {
    Const(String, Value),
    Var(String, Value),
    Scope,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    stack: VecDeque<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn pop(&mut self) -> Option<Symbol> {
        self.stack.pop_front()
    }

    pub fn push(&mut self, symbol: Symbol) {
        self.stack.push_front(symbol);
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.stack.iter().find(|symbol| match symbol {
            Symbol::Const(n, _) => n == name,
            Symbol::Var(n, _) => n == name,
            Symbol::Scope => false,
        })
    }
}
