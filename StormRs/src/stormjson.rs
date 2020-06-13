#![forbid(unsafe_code)]
#[serde(deny_unknown_fields)]

use std::{
	prelude::v1::*,
	io::prelude::*,

	collections::BTreeMap,

	f64, i64, usize, u64, u8,

	option::Option,

	io::{ Error, Result },
	
	fmt::{ Display, Formatter },
};

use serde::{
	ser,
	Serialize,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Consts {
	OBJECT,
	NAME,
	NUMBER,
	BOOLEAN,
	ARRAY,
	NULL,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Json {
	object : BTreeMap<String, Consts>,
	name : String,//what about Vec<Task> ... ?
	number : f64,
	boolean : bool,
	array : Vec<i64>,
	null : usize,//or None:= _
}

impl Json {//do we need a careted variable?

	pub fn jsonobject(&self) -> &BTreeMap<String, Consts> {
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
			object => Consts::OBJECT,
			name => Consts::NAME,
			number => Consts::NUMBER,
			boolean => Consts::BOOLEAN,
			array => Consts::ARRAY,
			null => Consts::NULL,
		};
		//pub fn init(&self) -> Self
	}
}

#[derive(Serialize, Deserialize)]//#[serde(tag = "type")] can complement Java FFI-ing.
#[serde(tag = "type")]//#[serde(untagged)] is for varying enum' properties.
enum Message {
    Request { id: String, method: String, params: Params },
    Response { id: String, result: Value },
}//https://serde.rs/enum-representations.html

#[derive(Serialize, Deserialize, Debug)]
pub struct Serials {
	object : String,
	name : String,
	number : f64,
	boolean : bool,
	array : Vec<i64>,
	null : usize,
}

pub fn s_to_string<T>(value: T) -> Result<String>
where
T : Serialize,
{
	let mut s = Serials { object : String::new(), };

	value.serialize(&mut s)?;
	Ok(s.object)
}

impl<'a> ser::Serializer for &'a mut Serials {
	type Ok = ();
	type Error = Error;

	type SerializeSeq = Self;
	type SerializeStruct = Self;
	type SerializeTuple = Self;
	type SerializeMap = Self;

	fn serialize_bool(self, v: bool) -> Result<()> {
		self.object += if v { "true" } else { "false" };
		Ok(())
	}

	fn serialize_i64(self, v: i64) -> Result<()> {
		self.object += &v.s_to_string();
		Ok(())
	}

	fn serialize_f64(self, v: f64) -> Result<()> {
		self.object += &v.s_to_string();
		Ok(())
	}

	fn serialize_str(self, v: &str) -> Result<()> {
		self.object += "\"";
		self.object += v;
		self.object += "\'";
		Ok(())
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<()> {
		use serde::ser::SerializeSeq;
		let mut seq = self.serialize_seq(Some(v.len()))?;
		for byte in v {
			seq.serialize_element(byte)?;
		}

		seq.end()
	}
}//https://serde.rs/impl-serializer.html
/*
	TOODO:
		Need to create a iterator and mapper function.
	
	Be able to iterate through each type value with the lifetime-variable.
*/