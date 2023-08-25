use std::fmt;

use crate::vm::expression::VM as VM;
use crate::traits::h::HasBytecode;
use crate::traits::h::HasStack;
use crate::traits::h::HasValue;

#[derive(Debug)]
pub struct Card {
    name: String,
    bytecode: Vec<u8>,
    pub value: u8,
    stack: (u8, Vec<u8>),
}

impl Card {
    pub fn new(
               name: String,
               bytecode: Vec<u8>,
               stack_size: u8)
               -> Card {
        Card {
            name,
            bytecode,
            value: 0,
            stack: (0, vec![0; stack_size as usize]),
        }
    }

    pub fn resolve(&mut self) {
        self.run();
    }
}

impl HasBytecode for Card {
    fn bytecode(&self) -> &Vec<u8> {
        &self.bytecode
    }

    fn get_bytecode(&self) -> &Vec<u8> {
        &self.bytecode
    }

    fn get_bytecode_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bytecode
    }

    fn set_bytecode(&mut self, bytecode: Vec<u8>) {
        self.bytecode = bytecode;
    }
}

impl HasStack for Card {
    fn get_stack(&self) -> &(u8, Vec<u8>) {
        &self.stack
    }

    fn get_stack_mut(&mut self) -> &mut (u8, Vec<u8>) {
        &mut self.stack
    }

    fn set_stack(&mut self, stack: (u8, Vec<u8>)) {
        self.stack = stack;
    }

}

impl HasValue for Card {
    fn get_value(&self) -> u8 {
        self.value
    }

    fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    fn get_value_mut(&mut self) -> &mut u8 {
        &mut self.value
    }
}

impl VM<Card> for Card {
    fn run_specific(&mut self, instruction: u8) {
        match instruction {
            _ => {
                println!("Unknown instruction: {}", instruction);
            }
        }
    }
}

impl  Clone for Card {
    fn clone(&self) -> Card {
        Card {
            name: self.name.clone(),
            bytecode: self.bytecode.clone(),
            value: self.value,
            stack: (self.stack.0, self.stack.1.clone()),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

