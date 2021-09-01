// Implementation of count-min sketch data structure

// Struct that has two things

// consume(item) -> add to the respective counters

// query(item) -> return the (approximate) count of item

use md5;
extern crate hex;

pub struct CountMinSketch {
	hash_tables: [[u64; 256]; 4],
}

impl CountMinSketch {
	pub fn new() -> CountMinSketch {
		CountMinSketch {hash_tables: [[0; 256]; 4]}
	}

	pub fn consume(&mut self, value: &str) -> bool {
		let digest = md5::compute(value);
		let digest_str = format!("{:x}", digest);

		let decoded = hex::decode(digest_str).expect("Decoding failed");

		// Get the min count for conservative updates
		let mut v = Vec::new();
		for i in 0..4 {
			v.push(self.hash_tables[i][usize::from(decoded[i])]);
		}
		let min_val = *v.iter().min().unwrap();

		for i in 0..4 {
			if self.hash_tables[i][usize::from(decoded[i])] == min_val {
				self.hash_tables[i][usize::from(decoded[i])] += 1;
			}
		}

		true
	}

	pub fn query(&self, value: &str) -> u64 {
		let digest = md5::compute(value);
		let digest_str = format!("{:x}", digest);

		let decoded = hex::decode(digest_str).expect("Decoding failed");

		let mut ret = u64::MAX;
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

    #[test]
    fn heavy_hitters() {
    	fn freq(value: u16) -> u16 {
    		if value > 9000 {
    			(value - 9000).pow(2)
    		} else {
    			(((value - 1) as f64/1000f64).floor() as u16) + 1
    		}
    	}

    	// Yo dawg...
    	assert_eq!(1, freq(1));
    	assert_eq!(1, freq(1000));
    	assert_eq!(2, freq(1001));
    	assert_eq!(9, freq(9000));
    	assert_eq!(1, freq(9001));
    	assert_eq!(2500, freq(9050));

    	let mut cms = CountMinSketch::new();
    	let mut count = 0 as u64;
    	for i in 1..9051 {
    		count += freq(i) as u64;
    		let istr = format!("{:x}", i);
    		for _ in 1..freq(i) {
    			cms.consume(&istr);
    		}
    	}

    	println!("{}", count);
    	for i in 1..9051 {
    		let istr = format!("{:x}", i);
    		if cms.query(&istr) > count/100 {
    			println!("{}", i);
    		}
    	}
    }
}