use rocket_contrib::json::{Json, JsonValue};
use std::io::Read;
use rocket::{Request, Data, Outcome, Outcome::*, Response};
use rocket::response::Body;
use std::io::Cursor;
use rocket::request::{self, FromRequest};
use rocket::data::{self, FromDataSimple};
use rocket::http::{Status, ContentType};
use serde_derive::{Deserialize, Serialize};
use serde::de::{Deserialize, Deserializer};
use regex::Regex;
use super::database::database;


#[derive(Serialize, Deserialize)]
pub struct TimeStampQuery {
    pub id: String,
    pub collection: String,
}

impl TimeStampQuery {
    pub fn is_valid(&self) -> bool {
        let re = Regex::new(r"^[A-F0-9]{76}$").unwrap();
        if !re.is_match(self.id.as_str()) {
            return false;
        }
        return true;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub timestamp: u64,
    pub collection: String,
    pub record: RecordItem
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordItem {
    pub info: Option<String>,
    pub codename: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "type")]
    pub type_name: Option<String>,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerRecordItem {
    pub info: Option<String>,
    pub codename: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "type")]
    pub type_name: Option<String>,
    pub id: String,
    pub timestamp: u64,
    pub ip: String,
    pub location: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerRecord {
    pub timestamp: u64,
    pub collection: String,
    pub record: ServerRecordItem
}

pub fn simple_response<'a>(content: String, content_type: ContentType, status: Status) -> Response<'a> {
    let length = content.len();
    Response::build().raw_body(Body::Sized(Cursor::new(content), length as u64))
        .header_adjoin(content_type)
        .status(status)
        .finalize()
}