#![allow(
	non_upper_case_globals,
	non_camel_case_types,
	non_snake_case,
)]

use std::{
	prelude::v1::*,

	rc::Rc,

	boxed::Box,
};

pub struct Client<Message> {
	message_value : Message,
}

impl<Message> Client<Message> {

	pub fn create<A, B>(&self, x : A) -> impl Fn(&self, &str) -> Result<(A, B, &str), &str> where
																								A : Fn(&self, &str) -> Result<(B, &str), &str> {
		//let link : &str;
		let a_ = &self.message_value;
		let val = move |a_| match x(a_) { Some(data) if data => Ok((a_, data)), _ => Err((a_, data)) };

		val
	} /* Fn(&self, &str) so that the inputted function is the Client structure itself,
	whilst indicating it as a string-slice; and the desired output being Result<(A, B, &str)> := which is a unit. */

	pub fn lift<A, B, X, Y>(x : A, y : B) where
												A : Fn(X) -> Result<B, Y> {
		// To-do: ...
		let &mut socket : std::net::SocketAddrV4;
		let &mut socket = &str;
		let x_a_ = move |a_| x(a_) = Client::create(a_, socket);
		// match x_a_ { Some(data) if data => Ok((x_a_, data)), _ => Err((x_a_, data)) }
	}

	pub fn update<A, B>(&self, x : A) where
											A : Fn(&self) -> Result<(), B> {}

	pub fn delete<A, B, X, Y>(&self, x : A, y : B) where
														B : Fn(Y) -> Result<A, X> {}
}

#[derive(Debug)]
pub struct Queue(Box::<u64>);

impl Queue {
	fn queue_too<Data>(&self) -> Result<(),Data> {
		let Queue(something) = self;
		// To-do: Possibly do a pattern-match. Initialize a new Vec<> and transpose its buffer as a stream.
		Ok(())
	}
}

trait Sink {
	fn process<Source, Message, Function>(_src : Source, _msg : Message, _fn : Function) where
																								Source : Fn(Function) -> Result<(), Message> {
		let (_src, _msg) : (String, String);

	}

	fn handle<Source, Message, Function>(_src : Rc<Source>, _fn : Function)  where
																					Source : Fn(Function) -> Result<(), Message> {
		let msg = Message;

		// TODO ...
	}

	fn help<Function>(_fn : Function,) {}

	fn operate<Source, Function>(_src : Source, Sink::help(_fn) : Function) -> Result<(), Function> {
		f = Sink::help(_fn); Ok((f)) }

	fn erron<Message>(_msg : Message) -> Result<(), Message> { Err((_msg)) }
}

pub trait Transform {
	fn transform<Source, Message, Function>(_src : Source, _msg : Message, _fn : Function) where
																								Source : Fn(Function) -> Result<(), Message> {
		// To-do: ...
		let (_src, _msg) : (String, String);
	}
}

impl Transform for Sink {
	// To-do: ... Create an Event/Message Queue function(s)
	fn transform(_src : Source, _msg : Message, _fn : Function) {

	}
}