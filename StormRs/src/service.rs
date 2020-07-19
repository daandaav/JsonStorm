#![allow(
	non_upper_case_globals,
	non_camel_case_types,
	non_snake_case,
)]

use std::{
	prelude::v1::*,

	rc::Rc,
};

pub struct Client<Message> {
	message_value : Message,
}

impl Client {

	pub fn create<A, B>(&self, x : A) -> impl Fn(&self, &str) -> Result<(A, B, &str), &str> where
																								A : Fn(&self, &str) -> Result<(B, &str), &str> {
		//let link : &str;
		let a_ = &self.message_value;
		let val = move |a_| match x(a_) { Some(data) if data => Ok((a_, data)), _ => Err(a_, data) };

		val
	} /* Fn(&self, &str) so that the inputted function is the Client structure itself,
	whilst indicating it as a string-slice; and the desired output being Result<(A, B, &str)> := which is a unit. */

	pub fn lift<A, B, X, Y>(x : A, y : B) -> impl Fn(A) -> Result<(B, Y)> where
																				A : Fn(X) -> Result<(B, Y)> {
		// To-do: ...
		let &mut socket : std::net::SocketAddrV4;
		let &mut socket = &str;
		let x_a_ = move |a_| x(a_) = Client::create(a_, socket);
		match x_a_ { Some(data) if data => Ok((x_a_, data)), _ => Err(x_a_, data) }
	}

	pub fn update<A, B>(&self, x : A) -> impl Fn(&self) -> Result<(B)> where
																			A : Fn(&self) -> Result<B> {}

	pub fn delete<A, B, X, Y>(&self, x : A, y : B) -> impl Fn(B) -> Result<(A, X)> where
																						B : Fn(Y) -> Result<(A, X)> {}
}

pub trait Sink {
	pub fn process<Source, Function, Message>(_src : Source, msg : Message) -> impl Fn(Function) -> Result<(Source, Message)> where
																																Source : Fn(Function) -> Result<Message> {
		let (_src, msg) : (&str, &str);
		move |msg| match _src.chars().next() { Some(_src) => Ok((&_src[msg.len()..])), _ => Err((_src(i), msg)) }
	}

	pub fn handle<Source, Message, Function>(_fn : Function, _src : Rc) -> impl Fn(Function) -> Result<Source, Message> where
																															Source : Fn(Function) -> Result<Message> {
		let _src : Source; let msg : Message; let _src = Rc<Source> { Sink::process(_src, msg) };
	}

	pub fn help<Function>(f : Function,) {}

	pub fn operate<Source, Function>(_src : Source, Sink::help(fnc) : Function) -> Result<> { Ok((_src, fnc)) }

	pub fn erron<Message>(msg : Message) -> Result<> { Err((msg)) }
}

pub trait Transform {
	pub fn handle<>() {}

	pub fn process<>() {}
}