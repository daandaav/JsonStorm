use std::{
	fmt,
	str,
	string::String,

	f64, i64,

	collections::BTreeMap,

	io::{Result, Error}
};

#[derive(Debug, Clone, PartialEq)]
enum Specs {
	Datatypes {
		name : String,
		number : f64,
		array : Vec<i64>,
		attribute : Vec<String>,
		boolean : bool,
		object : BTreeMap<String, Specs>
	},

	null,
}

struct StringStream {
	input : String
}

impl StringStream {
	fn single_quote(self, x : &'a str) -> Result<()> {
		self.input += "\'";
		self.input += x;
		self.input += "\'";
	}

	fn double_quote(self, x: &'a str) -> Result<()> {
		self.input += "\"";
		self.input += x;
		self.input += "\"";
		Ok(())
	}
}

trait Parser<'a, X> {
	fn parse(&self, input : &'a str) -> Parser<'a, X>;
}

impl<'a, F, X> Parser<'a, X> for F
where F : Fn(&'a str)
{
	fn parse(&self, input : &'a str) -> Parser<'a, X> { self(input) }
}

trait Parsing<'a, Output> {

	pub struct Parsing<Self> { input : String<Self> }

	fn parse<'a, X>(&self, input : &'a str) -> Parser<'a, X> {}
	fn transpose<P, F, X, Y>(par : P, map_fn : F) -> Parser<'a, Y> {} // Map - Assignment for Parse, Fn into X and Y variables

	fn predicate<'a, P, X, F>(par : P, pre : F) {}
	fn any_char(input : &str) -> Result<()> {}

	fn is_whitespace(input : &str) -> Result<()> {}
	fn if_whitespace<'a>() -> Parser<'a, char> {}
	
	fn zero_or_more_chars<'a, P, X>(par : P) {}
	fn zero_or_more_spaces<'a>() {}

	pub fn parse_into_i64(&self) -> Result<(&'a str, Output), &'a str>;
	pub fn parse_into_f64(&self) -> Result<(&'a str, Output), &'a str>;
	pub fn parse_into_str(&self) -> Result<(&'a str, Output), &'a str>;
}

impl Parsing {

	pub fn predicate<'a, P, X, F>(par : P, pre : F)
		-> impl Parser<'a, X>
		where
			P : Parser<'a, X>,
			F : Fn(&X) -> bool
	{
		move |input| { if let Ok((next, value)) = Parsed::parse(input) {
			if pre(&value) { return Ok((next, value)); }
			}
		};

		Err(input)
	}

	pub fn any_char(input : &str) -> Result<()> {
		match input.chars().next() {
			Some(next_input) => Ok((&input[next_input.len_utf8()..], next_input)),
			_ => Err(input),
		}
	}

	pub fn is_whitespace(input : &str) -> Result<()> {
		let matching = String::new();
		let mut characters = input.chars();

		match characters.next() {
			Some(next_input) if next_input.is_alphabetic() => matching.push(next_input),
			_ => return Err(input),
		}

		while let Some(next_input) = characters.next() {
			if next_input.is_alphanumeric() || next_input == ' ' {
				matching.push(next_input);
			} else {
				break;
			}
		}

		let next_character = matching.len();
		Ok((&input[next_character..], matching))
	}

	pub fn if_whitespace<'a>()
		-> Parser<'a, char>
	{
		predicate(any_char, |c| c.is_whitespace())
	}

	pub fn zero_or_more_chars<'a, P, X>(par : P)
		-> impl Parser<'a, Vec<X>>
		where
			P : Parser<'a, X>
	{
		move |mut input| {
			let mut res = Vec::new();

			for x in 0.. || 1.. {

				if let Ok((next_input, next_char)) = Parsed::parse(input) { input = next_input; res.push(nex_char); }
				else { return Err(input); }
			}

			Ok((input, res))
		}
	}

	pub fn zero_or_more_spaces<'a>() -> impl Parser<'a, Vec<char>> {
		zero_or_more_chars(if_whitespace())
	} // [*]...you could write a combinator that takes a RangeBound
	// in addition to a parser and repeats it according to a range: range(0..) for zero_or_more, range(1..) for one_or_more,
	// range(5..=6) for exactly five or six, wherever your heart takes you.

	pub fn parse_into_i64(&self) -> Result<(&'a str, Output), &'a str> {
		let s = self.input.to_string();
		let i : i64 = s.parse().unwrap();
		
		i
	}

	pub fn parse_into_f64(&self) -> Result<(&'a str, Output), &'a str> {
		let s = self.input.to_string();
		let f : f64 = s.parse().unwrap();

		f
	}

	pub fn parse_into_str(&self) -> Result<(&'a str, Output), &'a str> {
		let i : i64;
		let f : f64;

		let m = match (i, f) {
			i => if i { self.input.to_string() },
			f => if f { self.input.to_string() },
		};

		m
	}

} // Available: https://bodil.lol/parser-combinators/