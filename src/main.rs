// Implementation of count-min sketch data structure

// Struct that has two things

// consume(item) -> add to the respective counters

// query(item) -> return the (approximate) count of item

use md5;


fn main() {
	let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    println!("The hash is {:x}", digest);
}


