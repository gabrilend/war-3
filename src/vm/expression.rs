use crate::traits::h::HasBytecode;
use crate::traits::h::HasStack;
use crate::traits::h::HasValue;
use crate::vm::instruction::Instruction;
use crate::{green, red};

/* stack is a tuple - the first value (stack[0]) is the index of the next */
/* empty spot at the top of the stack, the second value (stack[1]) is the */
/* actual stack */
pub trait VM<T: HasBytecode + HasStack + HasValue>:
                HasBytecode + HasStack + HasValue {
    const MAX_STACK: u8 = 20;

    fn run_specific(&mut self, instruction: u8);

    fn run(&mut self) {
        let mut i = 0;
        while i < self.bytecode().len() {
            let instruction: u8 = self.bytecode()[i];
            match instruction {

                x if x == Instruction::InstLiteral as u8 => {
                    i += 1;
                    println!("Literal value: {}", self.bytecode()[i]); 
                    self.push(self.bytecode()[i].clone());
                }
                x if x == Instruction::InstAddValue as u8 => {
                    println!("Adding");
                    let a: u8 = self.pop();
                    let b: u8 = self.pop();
                    println!("Adding {} and {}", a, b); 
                    self.push(a + b);
                }
                    

                x if x == Instruction::InstSubValue as u8 => {
                    let a: u8 = self.pop();
                    let b: u8 = self.pop();
                    println!("Subtracting {} and {}", a, b);
                    self.push(a - b);
                    
                }

                x if x == Instruction::InstMulValue as u8 => {
                    println!("Multiplying");
                    let a: u8 = self.pop();
                    let b: u8 = self.pop();
                    println!("Multiplying {} and {}", a, b);
                    self.push(a * b);
                }
                x if x == Instruction::InstDivValue as u8 => {
                    let a: u8 = self.pop();
                    let b: u8 = self.pop();
                    println!("Dividing {} and {}", a, b);
                    self.push(a / b);
                }

                _ => {
                    self.run_specific(instruction);
                    break;
                }
            }

            i += 1;
        }
    }

    /* alright it's official chatgpt knows more than me */
    /* stack.0 is the index of the next empty spot at the top of the stack */

    fn push(&mut self, value: u8) {
        assert!(self.stack().0 < self.stack().1.len() as u8);
        println!("Pushing {} to stack", green!("{}", value.to_string()));
        let mut_stack = self.get_stack_mut();
        mut_stack.0 += 1;
        mut_stack.1[mut_stack.0 as usize] = value;
    }

    fn pop(&mut self) -> u8 {
        assert!(self.stack().0 > 0);
        let popped: u8 = self.stack().1[self.stack().0 as usize];
        self.get_stack_mut().0 -= 1;
        println!("Popping {} from stack", red!("{}", popped.to_string()));
        popped
    }
}

