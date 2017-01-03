#![allow(non_snake_case)]
#![feature(proc_macro)]
#![feature(custom_attribute)]

extern crate cookie;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate regex;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate url;

#[macro_use] pub mod macros;
pub mod client;
pub mod protocol;
pub mod error;
pub mod server;


#[cfg(test)]
mod nullable_tests {
    use super::common::Nullable;

    #[test]
    fn test_nullable_map() {
        let mut test = Nullable::Value(21);

        assert_eq!(test.map(|x| x << 1), Nullable::Value(42));

        test = Nullable::Null;

        assert_eq!(test.map(|x| x << 1), Nullable::Null);
    }

    #[test]
    fn test_nullable_into() {
        let test: Option<i32> = Nullable::Value(42).into();

        assert_eq!(test, Some(42));
    }
}
