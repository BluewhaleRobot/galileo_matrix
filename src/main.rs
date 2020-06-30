#![feature(proc_macro_hygiene, decl_macro)]

mod models;
mod database;
mod server;
mod req_guards;

use server::init_server;
fn main() {
    init_server().launch();
}
