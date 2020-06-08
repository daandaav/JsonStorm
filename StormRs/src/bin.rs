#![forbid(unsafe_code)]
//mods [?]
mod lib;
mod stormjson;

fn main() {
	println!("JSON Strom: Storm (in Rust) Interceptor");

	stormjson::Bitmap::test_bitmapping();
}