use std::{
	prelude::v1::*,
	io::prelude::*,

	collections::BTreeMap,

	f64, i64, u64,

	option::Option,
	result::Result,
	fmt::Display,
};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum JsonFi {
	Object,
	Name,
	Number,
	Boolean,
	Array,
	Null,
}

pub struct JsonSt<T> {
	object : BTreeMap<String, JsonFile>,
	name : String,//what about Vec<Task> ... ?
	number : f64,
	boolean : bool,
	array : Vec<T>,
	null : None,//or None:= _
}

impl JsonSt {//do we need a careted variable?
	fn new(&self) -> JsonSt {
		self.object = BTreeMap::new(),
		self.name = String::from(),
		self.number = f64::is_normal(),//then...TODO: f64::to_bits(u64) - bitmapping; maybe typecast f64 as u64 [?]
		self.boolean = false,
		self.array = Vec::new(),
		self.null = None,//:=_
	}

	fn init(self) -> Option<JsonFi> {
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
			self.null => JsonFi::Null,
		};

		let mut map = BTreeMap::new();
		map.insert(log_stamp, json_data);

		assert_eq!(map.get(&log_stamp), Some(json_data));//json_name, Some(json_object)
	}
}

pub struct Data {
	bits : Vec<u8>,
}

impl Data {
	fn throughputself(self) -> Self {
		let self.buffer = Vec::with_capacity(c);

		let n : u64;

		let b = n.to_le_bytes();//n.to_littleendian_bytes()

		for _ in 0..c { self.buffer.push(b) }
	}
}
/*
	TOODO:
		Need to create a iterator and mapper function.
	
	Be able to iterate through each type value with the lifetime-variable.
*/