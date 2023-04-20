#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod db;
mod employees;
mod error_handler;
mod schema;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
}
