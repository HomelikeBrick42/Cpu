pub struct Memory {
    memory: Vec<u8>,
}

// im making a few assumptions when converting between u64 and usize
const _: () = assert!(std::mem::size_of::<usize>() >= std::mem::size_of::<u64>());
const _: () = assert!(std::mem::size_of::<u64>() == 8);

impl Memory {
    pub fn new(size: u64) -> Memory {
        Memory {
            memory: vec![0; size as usize],
        }
    }

    pub fn read(&self, address: u64) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_u64(&self, address: u64) -> u64 {
        let bytes = std::array::from_fn(|i| self.read(address + i as u64));
        u64::from_le_bytes(bytes)
    }

    pub fn write(&mut self, address: u64, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn write_u64(&mut self, address: u64, value: u64) {
        for (i, value) in u64::to_le_bytes(value).into_iter().enumerate() {
            self.write(address + i as u64, value);
        }
    }
}
