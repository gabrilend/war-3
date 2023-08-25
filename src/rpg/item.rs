

pub struct Item {
    pub name: String,
    pub description: String,
    pub quantity: u8,
}

impl Item {
    pub fn new(name: String, description: String, quantity: u8) -> Item {
        Item {
            name,
            description,
            quantity,
        }
    }
}
