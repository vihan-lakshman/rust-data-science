// Implementation of count-min sketch data structure

// Struct that has two things

// consume(item) -> add to the respective counters

// query(item) -> return the (approximate) count of item

use md5;

pub struct CountMinSketch {
	hash_tables: [[u8; 256]; 4],
}

impl CountMinSketch {
	pub fn new() -> CountMinSketch {
		CountMinSketch {hash_tables: [[0; 256]; 4]}
	}

	pub fn consume(&mut self, value: &u8) -> bool {
		true
	}

	pub fn query(&self, value: &u8) -> u8 {
		1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn basic_test() {
    	let inputs = [1,2,3,4,5,6,7,8,9,10];
    	let mut CMS = CountMinSketch::new();
    	for item in inputs.iter() {
    		assert!(CMS.consume(item))
    	}
    	for item in inputs.iter() {
    		assert_eq!(1, CMS.query(item))
    	}
    }
}