#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket_contrib;

mod block;
mod server;
mod sha;
mod chain;
mod response;
mod transaction;

use block::*;
use server::*;
use sha::*;
use chain::*;
use response::*;
use transaction::*;


fn main() {
    println!("Hello, Blockchain!");
    server::rocket().launch();
}