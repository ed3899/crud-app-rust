use crate::employees::{}
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

pub fn init_routes(config: &mut web::ServiceConfig) {}
