pub trait HasValue: Clone {
    fn get_value(&self) -> u8;
    fn set_value(&mut self, value: u8);
}
