#![forbid(unsafe_code)]

use std::{
	prelude::v1::*,
	io::prelude::*,

	collections::BTreeMap,

	f64, i64, u64,

	option::Option,
	result::Result,
	fmt::{ Display, Error, Formatter },
};

#[derive(Debug, Clone, PartialEq)]//Serialize and Deserialize [?]
pub enum JsonFi {
	Object,
	Name,
	Number,
	Boolean,
	Array,
	Null,
}

#[derive(Debug, Clone)]
pub struct JsonSt<T> {
	object : BTreeMap<String, JsonFi>,
	name : String,//what about Vec<Task> ... ?
	number : f64,
	boolean : bool,
	array : Vec<T>,
	null : None,//or None:= _
}

impl JsonSt<T> {//do we need a careted variable?
	pub fn new(&self) -> JsonSt {
		self.object = BTreeMap::new(),
		self.name = String::from(),
		self.number = f64::is_normal(),//then...TODO: f64::to_bits(u64) - bitmapping; maybe typecast f64 as u64 [?]
		self.boolean = false,
		self.array = Vec::new(),
		self.null = None,//:=_
	}

	pub fn jsonobject(&self) -> &BTreeMap<String, JsonFi> {
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

	pub fn jsonarray(&self) -> &Vec<T> {
		&self.array
	}

	pub fn jsonnull() {
		return None;
	}

	pub fn init(self) -> Option<JsonFi> {
		//TODO: JsonSt into JsonFi into DataFi
		let json_data = match (
			self.object,
			self.name,
			self.number,
			self.boolean,
			self.array,
			self.null,
		){
			self.object => JsonFi::Object,
			self.name => JsonFi::Name,
			self.number => JsonFi::Number,
			self.boolean => JsonFi::Boolean,
			self.array => JsonFi::Array,
			(self.null, _) if { !_ => JsonFi::Null } else { unimplemented!() },
		};

		let mut map = BTreeMap::new();
		map.insert(log_stamp, json_data);

		assert_eq!(map.get(&log_stamp), Some(json_data));//json_name, Some(json_object)
	}
}

pub struct Data {
	bits : Vec<u64>,
}

impl Data {

	pub fn new(&self) {
		&self.bits = Vec::new()
	}

	pub fn le_into(self) -> Self {
		let c : u64;

		let Data.bits = Vec::with_capacity(c);

		for _ in 0..c {
			let n : u64;
			let b = n.to_le_bytes(|a|);//n.to_littleendian_bytes()
		}

		Data.bits.push(b)
	}
}

pub struct Bitmap {
	buffer : Vec<u64>,
}

impl Bitmap {
	fn bitmap(self) {
		let n : usize;

		let buffer = Vec::with_capacity(n);

		for _ in 0..n {
			self.buffer.push(256)
		}

		let from_bits = u64::from_le_bytes(Data.bits);

		let to_bits = from_bits / 8 + if from_bits % 8 > 0 { 1 } else { 0 }
	}

	fn read_le_u64(input: &mut &[u8]) -> u64 {
		let (int_bytes, rest) = input.split_at(std::mem::size_of::<u64>());
		*input = rest;
		u64::from_le_bytes(int_bytes.try_into().unwrap())
	}//taken from: https://doc.rust-lang.org/std/primitive.u64.html#method.from_le_bytes
}

#[cfg(test)]
pub mod tests {
	#[test]
	pub fn test_bitmapping() {
		let b = Bitmap::bitmap();
		let r = b % 64;

		let l = if r == 0 {
			b
		} else {
			b + 64 - r
		};

		assert_eq!(match b {
			b => r
		}, l);
	}
}
/*
	TOODO:
		Need to create a iterator and mapper function.
	
	Be able to iterate through each type value with the lifetime-variable.
*/