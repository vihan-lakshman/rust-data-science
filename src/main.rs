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
		let digest_str = format!("{:x}", digest);

		let decoded = hex::decode(digest_str).expect("Decoding failed");

		for i in 0..4 {
			self.hash_tables[i][usize::from(decoded[i])] += 1;
		}

		true
	}

	pub fn query(&self, value: &str) -> u8 {
		let digest = md5::compute(value);
		let digest_str = format!("{:x}", digest);

		let decoded = hex::decode(digest_str).expect("Decoding failed");

		let mut ret = 255;
		for i in 0..4 {
			let val = self.hash_tables[i][usize::from(decoded[i])];
			if val < ret { ret = val;}
		}
		ret

	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn basic_test() {
    	let inputs: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
    	let mut cms = CountMinSketch::new();
    	for item in inputs.iter() {
    		assert!(cms.consume(item));
    	}

    	for item in inputs.iter() {
    		assert_eq!(1, cms.query(item));
    	}

    	let inputs2: [&str; 10] = ["1", "1", "1", "1", "1", "1", "1", "1", "1", "10"];
    	for item in inputs2.iter() {
    		assert!(cms.consume(item))
    	}

    	assert_eq!(10, cms.query("1"));
    	assert_eq!(2, cms.query("10"));
    }
}