#![allow(
	non_upper_case_globals,
	non_camel_case_types,
	non_snake_case,
)]

use std::prelude::v1::*;

#[derive(Debug, Clone)]
pub struct Spout<T> {
	//Spout Properties
	name : String,
	output : Arc<Mutex<T>>,
}

impl<T> Spout {
	pub fn new<T>(&self, o : Arc<Mutex<T>>) -> Self {
		let spn = Arc::new(Mutex::new( // Spout Arc::new(Mutex::new(Spout))
			Spout {
				name : String::from(),
				output : o,
			}
		));

		spn
	}
}

enum IERdfEvent {
	//To-Do: ...
}

trait Rdf {

	type ProductEvent = (IERdfEvent);

	type InputTrigger = (String, &str);

	type OutputSender = (String, i64);

	pub fn decode(x : DomainEvent) -> Result<()>;

	pub fn handle(x : InputTrigger) -> Result<()>;

	pub fn interpret(x : OutputSender) -> Result<()>;

	pub fn process() -> Result<()>;
}

impl Rdf {
	pub fn decode(x : ProductEvent) -> Result<()> {
		let m = match x {
			Some(x) => Fn(x),
			_ => None,
		};

		m
	}

	pub fn handle(x : InputTrigger) -> Result<()> {
		Fn(x);

		Ok(())
	}

	pub fn interpret(x : OutputSender) -> Result<()> {
		let m = match x {
			Some(x) => if x { let d : Vec::new(); }
				else { let s = d.iter() .push();  }
		};

		m;

		Ok(())
	}

	pub fn process() -> Result<()> {
		
		Ok(())
	}
}