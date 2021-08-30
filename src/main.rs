// Implementation of count-min sketch data structure

// Struct that has two things

// consume(item) -> add to the respective counters

// query(item) -> return the (approximate) count of item

use md5;
extern crate hex;

pub struct CountMinSketch {
	hash_tables: [[u8; 256]; 4],
}

impl CountMinSketch {
	pub fn new() -> CountMinSketch {
		CountMinSketch {hash_tables: [[0; 256]; 4]}
	}

	pub fn consume(&mut self, value: &str) -> bool {
		let digest = md5::compute(value);
		let bytes: [u8; 4] = [digest[0], digest[1], digest[2], digest[3]];
		println!("{:x}", digest);
		// let decoded = hex::decode(digest).expect("Decoding failed");
		true
	}

	pub fn query(&self, value: &str) -> u8 {
		1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn basic_test() {
    	let inputs: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
    	let mut CMS = CountMinSketch::new();
    	for item in inputs.iter() {
    		assert!(CMS.consume(item))
    	}
    	for item in inputs.iter() {
    		assert_eq!(1, CMS.query(item))
    	}
    }
}