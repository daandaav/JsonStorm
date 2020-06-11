#![forbid(unsafe_code)]

use std::{
	prelude::v1::*,
	io::prelude::*,

	collections::BTreeMap,

	f64, i64, usize, u64, u8,

	option::Option,
	result::Result,
	fmt::{ Display, Error, Formatter },
};

#[derive(Debug, Clone, PartialEq)]
enum JsonEnum {
	OBJECT,
	NAME,
	NUMBER,
	BOOLEAN,
	ARRAY,
	NULL,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Json {
	object : BTreeMap<String, JsonEnum>,
	name : String,//what about Vec<Task> ... ?
	number : f64,
	boolean : bool,
	array : Vec<i64>,
	null : usize,//or None:= _
}

impl Json {//do we need a careted variable?

	pub fn jsonobject(&self) -> &BTreeMap<String, JsonEnum> {
		&self.object
	}

	pub fn jsonstring(&self) -> &String {
		&self.name
	}

	pub fn jsonnumber(&self) -> &f64 {
		&self.number
	}

	pub fn jsonbool(&self) -> &bool {
		&self.boolean
	}

	pub fn jsonarray(&self) -> &Vec<i64> {
		&self.array
	}

	pub fn init(&self) {
		//TODO: JsonSt into JsonFi into DataFi
		let json_data = match (
			self.object,
			self.name,
			self.number,
			self.boolean,
			self.array,
			self.null,
		){
			object => JsonEnum::OBJECT,
			name => JsonEnum::NAME,
			number => JsonEnum::NUMBER,
			boolean => JsonEnum::BOOLEAN,
			array => JsonEnum::ARRAY,
			null => JsonEnum::NULL,
		};
		//pub fn init(&self) -> Self
	}
}

pub struct Data {
	bits : u64,
}

impl Data {
	pub fn le_into(self) {
		let b = self.bits.to_le_bytes();//n.to_littleendian_bytes()
	}
}

pub struct Bitmap {
	array : [u8; 8],
	buffer : Vec<usize>,
}

impl Bitmap {
	fn bitmap(self) {
		let mut n = 256u64;
		let b = Vec::<u64>::with_capacity(n as usize);

		for _ in 0..n {
			self.buffer.push(1)
		}

		let from_bits = u64::from_le_bytes(self.array);

		let to_bits = from_bits / 8 + if from_bits % 8 > 0 { 1 } else { 0 };
	}

	fn read_le_u64(input: &mut &[u8]) -> u64 {
		let (int_bytes, rest) = input.split_at(std::mem::size_of::<u64>());

		*input = rest;

		let arr : [u8; 8];

		arr.into_iter();
		u64::from_le_bytes(arr)
	}//taken from: https://doc.rust-lang.org/std/primitive.u64.html#method.from_le_bytes
}

#[cfg(test)]
mod tests {
	#[test]
	pub fn test_bitmapping() {
		let a : [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

		let f = u64::from_le_bytes(a);
		
		let t = f / 8 + if f % 8 > 0 { 1 } else { 0 };
		let r = t % 64; 

		let l = if r == 0 {
			t
		} else {
			t + 64 - r
		};

		assert_eq!(match f {
			f => r
		}, l);
	}
} 
/*
	TOODO:
		Need to create a iterator and mapper function.
	
	Be able to iterate through each type value with the lifetime-variable.
*/