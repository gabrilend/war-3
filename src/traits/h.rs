pub trait HasBytecode {
    fn get_bytecode(&self) -> &Vec<u8>;
    fn get_bytecode_mut(&mut self) -> &mut Vec<u8>;
    fn set_bytecode(&mut self, bytecode: Vec<u8>);
    fn bytecode(&self) -> &Vec<u8> {
        self.get_bytecode()
    }
}

pub trait HasStack {
    fn get_stack(&self) -> &(u8, Vec<u8>);
    fn get_stack_mut(&mut self) -> &mut (u8, Vec<u8>);
    fn set_stack(&mut self, stack: (u8, Vec<u8>));
    fn stack(&self) -> &(u8, Vec<u8>) {
        self.get_stack()
    }
}

pub trait HasValue {
    fn get_value(&self) -> u8;
    fn get_value_mut(&mut self) -> &mut u8;
    fn set_value(&mut self, value: u8);
    fn value(&self) -> u8 {
        self.get_value()
    }
}
