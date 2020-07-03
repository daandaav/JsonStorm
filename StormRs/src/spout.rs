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
	}
}