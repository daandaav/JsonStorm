#![forbid(unsafe_code)]
#[serde(deny_unknown_fields)]

use std::{
	prelude::v1::*,
	io::prelude::*,

	boxed::Box,
	collections::BTreeMap,

	f64, i64,

	thread,

	time::Duration,

	option::Option,

	sync::{ Arc, Mutex },

	io::{ Error, Result },
};

use serde::{
	ser,
	Serialize,
};

#[derive(Debug, Clone, PartialEq)]
pub enum SPECS {
	OBJECT,
	NAME,
	NUMBER,
	BOOLEAN,
	ARRAY,
	NULL,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Json {
	object : BTreeMap<String, SPECS>,
	name : String,//what about Vec<Task> ... ?
	number : f64,
	boolean : bool,
	array : Vec<i64>,
	null : None,//or None:= _
}

impl Json {//do we need a careted variable?

	pub fn jsonobject(&self) -> &BTreeMap<String, SPECS> {
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

	pub fn jsonnull(&self) -> None {
		&self.null
	}

	pub fn intro(&self) -> Result<(String, Self), String> {
		//TODO: JsonSt into JsonFi into DataFi
		let js = match (
			self.object,
			self.name,
			self.number,
			self.boolean,
			self.array,
			self.null,
		){
			object => SPECS::OBJECT,
			name => SPECS::NAME,
			number => SPECS::NUMBER,
			boolean => SPECS::BOOLEAN,
			array => SPECS::ARRAY,
			null => SPECS::NULL,
		};
		//pub fn intro(&self) -> Result<(String, Self), String>
		js
	}
}

/*
	TODO(Parse):
	pub struct Parser:=
		ouput_input : String

	impl Parser:
		fn parse_out_in:
			let string_as_integer = output_input.parse::<T>().unwrap();
			//or
			let string_as_integer_alternative : T = output_input.parse().unwrap();
*/

struct Parser {
	input : String,
}

impl Parser {
	type Parse<String, Output> = Result<(String, Output), String>;
	// Available: https://bodil.lol/parser-combinators/

	fn parse_into_i64(&self) -> Result<(&str, ()), &str> {
		let s = self.input.to_string();
		let i : i64 = s.parse().unwrap();
		
		i
	}

	fn parse_into_f64(&self) -> Result<()> {
		let s = self.input.to_string();
		let f : f64 = s.parse().unwrap();

		f
	}

	fn parse_into_str(&self) -> Result<()> {
		let i : i64;
		let f : f64;

		let m = match (i, f) {
			i => if i { self.input.to_string() },
			f => if f { self.input.to_string() },
		};

		m
	}
}

#[derive(Serialize, Deserialize)]//#[serde(tag = "type")] can complement Java FFI-ing.
#[serde(tag = "type")]//#[serde(untagged)] is for varying enum' properties.
enum Message {
    Request { id: String, method: String, params: Params },
    Response { id: String, result: Value },
}//Available: https://serde.rs/enum-representations.html

#[derive(Serialize, Deserialize, Debug)]
pub struct Serials {
	object : String,
}

impl<'a> ser::Serializer for &'a mut Serials {
	type Ok = ();
	type Error = Error;

	type SerializeSeq = Self;

	fn s_to_string<T>(value: T) -> Result<String>
	where
		T : Serialize,
	{
		let mut s = Serials { object : String::new(), };

		value.serialize(&mut s)?;
		Ok(s.object)
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<()> {
		use serde::ser::SerializeSeq;
		let mut seq = self.serialize_seq(Some(v.len()))?;
		for byte in v {
			seq.serialize_element(byte)?;
		}

		seq.end()
	}

	fn serialize_str(self, v: &str) -> Result<()> {
		self.object += "\"";
		self.object += v;
		self.object += "\"";
		Ok(())
	}

	fn serialize_f64(self, v: f64) -> Result<()> {
		self.object += &v.s_to_string();
		Ok(())
	}

	fn serialize_bool(self, v: bool) -> Result<()> {
		self.object += if v { "true" } else { "false" };
		Ok(())
	}

	fn serialize_i64(self, v: i64) -> Result<()> {
		let f = Vec::from(i64); 
		self.object += f.push(&v.s_to_string());//Hmmm...?
		//How do we push this same value into self.object [?]
		Ok(())
	}//TODO: Serialize into an JSON Array Data-type

	fn serialize_unit(self) -> Result<()> {
		self.object += "null";
		Ok(())
	}

	fn serialize_none(self) -> Result<()> {
		self.serialize_unit()
	}

	fn btreemap_sequence(self, x: &str) -> Result<()> {
		let x = match (ptr, key, val) {
			val => BTreeMap.insert(key, val),
			val => if val { Some(ptr) } else if None { SPECS::NULL },
		};

		x
	}
}//Available: https://serde.rs/impl-serializer.html

/*
	TODO:
	1. PATTERN-MATCH The Serialization of the JSON Object Data-type/structure!
	2. [Continuously] re/build JSON String(s). Program a Builder.
	2.1. Re/building the object Binary Tree Map (BTreeMap) recursively from a pass Rust object.
*/
struct Rebuilder {
	j : Arc<Mutex<Json>>,//js for JavaScript or JSON Structure. Your preference.
}
//TODO(Option<'a>): Need to use the Option library and also track the lifetime of: 'a

impl Rebuilder {

	fn new(&self, j : Arc<Mutex<Json>>) -> Self {
		let bui = Builder {
			js = Arc::new(Mutex::new(Json))
		};

		bui
	}
	//TODO: Build all the given JSON items...
	//-And...! How can we pass the Json struct into this; in utilizing the BTreeMap module?
	fn build<T>(&self, j : Arc<Mutex<Json>>) -> Result<()> {
		let js = Arc::new(Mutex::new(Json {
			object : BTreeMap::new(),
			name : String::new(),//what about Vec<Task> ... ?
			number : f64::to_le_bytes,
			boolean : false,
			array : Vec::new(),
			null : None,//or None:= _
		}));

		let duration : Duration;

		let clone = js.clone();

		let ts = std::thread::spawn(move |i| {
			thread::sleep(duration);

			let clone_copy = clone.lock().unwrap();

			let lo = loop {
				let c = clone_copy;
				
				let op = match (i, c) {
					//TODO: ...
					//... Within each thread and pool we have a cloned JSON structure ...
					//... re/building
					Some(i, c) if i { c : String::new() } => SPECS::OBJECT,
					_ => None
				};
			};

			if lo > 2 { Ok(()) } else if l > 4 { Err(); break }

			lo
		});

		ts
	}
}