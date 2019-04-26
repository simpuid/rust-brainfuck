pub struct Memory {
    pub foreward: Vec<u8>,
    pub backward: Vec<u8>,
    pub pointer: isize,
}

impl Memory {
    pub fn new() -> Memory {
        return Memory {
            foreward: vec![0],
            backward: Vec::new(),
            pointer: 0,
        };
    }

    pub fn move_pointer(&mut self, delta: isize) {
        if delta == 0 {
            return;
        }
        self.pointer += delta;
        self.allocate();
    }

    pub fn allocate(&mut self) {
        while !(self.foreward.len() as isize > self.pointer) {
            self.foreward.push(0);
        }
        while !(self.backward.len() as isize >= -self.pointer) {
            self.backward.push(0);
        }
    }

    pub fn get_data(&self) -> u8 {
        let mut result = self.foreward.get(self.pointer as usize);
        if self.pointer < 0 {
            result = self.backward.get((-self.pointer - 1) as usize);
        }
        match result {
            Some(b) => (return *b),
            None => (0),
        }
    }

    pub fn set_data(&mut self, data: u8) {
        let mut result = self.foreward.get_mut(self.pointer as usize);
        if self.pointer < 0 {
            result = self.backward.get_mut((-self.pointer - 1) as usize);
        }
        match result {
            Some(b) => (*b = data),
            None => {
                self.allocate();
                self.set_data(data);
            }
        }
    }
}
