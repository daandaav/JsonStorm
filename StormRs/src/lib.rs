#![forbid(unsafe_code)]

use std::{
	prelude::v1::*,
	
	io::prelude::*,
	
	error::Error,

	thread,
};

#[cfg(test)]
pub mod tests {
    #[test]
    pub fn it_works() {
        assert_eq!(1 + 0, 1);
	}
}