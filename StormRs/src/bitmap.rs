mod stormjson;

use std::{
	prelude::v1::*,
	io::prelude::*,

	f64, i64, u64,

	convert::TryInto,//whatta 'bout convert::TryFrom [?]

	option::Option,
	result::Result,
	fmt::Display,

	ops::{
		BitAnd,
		BitOr
	},
};

pub struct Bitmap {
	buffer : Vec<Data>,
}

impl Bitmap for Data {
	fn bitmap(self) -> Self {
		let self.buffer = Vec::with_capacity(n);
		for _ in 0..n {
			self.buffer.push(256)
		}

		let from_bits = u64::from_le_bytes(DataFi::throughputself(DataFi.bits));

		let to_bits = from_bits / 8;

		if to_bits % 8 > 0 { 1 } else { 0 }
	}

	fn read_le_u64(input: &mut &[u8]) -> u64 {
		let (int_bytes, rest) = input.split_at(std::mem::size_of::<u64>());
		*input = rest;
		u64::from_le_bytes(int_bytes.try_into().unwrap())
	}//taken from: https://doc.rust-lang.org/std/primitive.u64.html#method.from_le_bytes
}

/*
TODO:
	Bitmap Index or B-Tree Index
*/