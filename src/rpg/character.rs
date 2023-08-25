use crate::rpg::item::Item;
use crate::vm::expression::VM;

pub struct Character {
    pub name: String,
    pub health: u8,
    pub attack: u8,
    pub defense: u8,
    pub speed: u8,
    pub level: u8,
    pub exp: u8,
    pub exp_to_next_level: u8,
    pub gold: u8,
    pub inventory: Vec<Item>,
    pub equipped: Vec<Item>,
    pub vm: VM,
}

impl Character {
    pub fn new(name: String) -> Character {
        Character {
            name,
            health: 100,
            attack: 10,
            defense: 10,
            speed: 10,
            level: 1,
            exp: 0,
            exp_to_next_level: 100,
            gold: 0,
            inventory: Vec::new(),
            equipped: Vec::new(),
            vm: VM::new(),
        }
    }
}

impl VM for Character {
    fn run_specific(instruction: u8) {
        println!("Running instruction: {}", instruction);
        

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
