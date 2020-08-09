#![feature(proc_macro_hygiene, decl_macro, plugin)]
extern crate rocket;
#[macro_use] extern crate rocket_codegen;
extern crate hmac;
extern crate sha2;

pub mod admin;
pub mod authentication;