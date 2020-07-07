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
pub enum JSON {
	OBJECT(BTreeMap<String, JSON>),
	NAME(String),
	NUMBER(f64),
	BOOLEAN(bool),
	ARRAY(Vec<i64>),
	NULL,
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
			val => if val { Some(ptr) } else if None { JSON::NULL },
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
	js : Arc<Mutex<JSON>>,//js for JavaScript or JSON Structure. Your preference.
}
//TODO(Option<'a>): Need to use the Option library and also track the lifetime of: 'a

impl Rebuilder {

	fn new(&self, js : Arc<Mutex<JSON>>) -> Self {
		let bui = Builder {
			j = Arc::new(Mutex::new(JSON))
		};

		bui
	}
	//TODO: Build all the given JSON items...
	//-And...! How can we pass the JSON struct into this; in utilizing the BTreeMap module?
	fn build<T>(&self, js : Arc<Mutex<JSON>>) -> Result<()> {
		let j = Arc::new(Mutex::new(JSON {
			object : BTreeMap::new(),
			name : String::new(),//what about Vec<Task> ... ?
			number : f64::to_le_bytes,
			boolean : false,
			array : Vec::new(),
			null : None,//or None:= _
		}));

		let duration : Duration;

		let clone = j.clone();

		let ts = std::thread::spawn(move |i| {
			thread::sleep(duration);

			let clone_copy = clone.lock().unwrap();

			let lo = loop {
				let c = clone_copy;
				
				let op = match (i, c) {
					//TODO: ...
					//... Within each thread and pool we have a cloned JSON structure ...
					//... re/building
					Some(i, c) if i { c : String::new() } => JSON::OBJECT,
					_ => None
				};
			};

			if lo > 2 { Ok(()) } else if l > 4 { Err(); break }

			lo
		});

		ts
	}
}