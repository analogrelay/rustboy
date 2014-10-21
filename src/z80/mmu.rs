pub struct Z80MMU {
	ram: [u8, ..8192], // 8K RAM
	//vid: [u8, ..8192], // 8K Video Memory
}

impl Z80MMU {
	pub fn new() -> Z80MMU {
		Z80MMU {
			ram: [0, ..8192],
			//vid: [0, ..8192]
		}
	}

	pub fn read_byte(&self, addr: u16) -> u8 {
		self.ram[addr as uint]
	}

	#[allow(unused_variable)]
	pub fn read_word(&self, addr: u16) -> u16 {
		unimplemented!()
	}

	pub fn write_byte(&mut self, addr: u16, val: u8) {
		self.ram[addr as uint] = val;
	}

	#[allow(unused_variable)]
	pub fn write_word(&mut self, addr: u16, val: u16) {
		unimplemented!()
	}
}