pub type Value = f64;

pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn free(&mut self) {
        self.values = Vec::new();
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn print_value(&self, index: u8) {
        print!("{}", self.values[index as usize]);
    }
}
