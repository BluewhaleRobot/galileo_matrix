use rocket::Response;
use rocket::response::Body;
use std::io::Cursor;
use rocket::http::{Status, ContentType};
use serde_derive::{Deserialize, Serialize};
use regex::Regex;


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
    pub timestamp: i64,
    pub collection: String,
    pub record: RecordItem,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordItem {
    pub id: String,
    pub codename: Option<String>,
    pub version: Option<String>,
    // exceptions
    #[serde(rename = "type")]
    pub type_name: Option<String>,
    pub info: Option<String>,
    // power
    pub power: Option<f32>,
    // navigation
    pub events: Option<Vec<NavEventItem>>,
    pub results: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerRecordItem {
    // basic
    pub id: String,
    pub timestamp: i64,
    pub ip: String,
    pub location: Option<PhyAddrInfo>,
    pub codename: Option<String>,
    pub version: Option<String>,
    // exceptions
    pub info: Option<String>,
    #[serde(rename = "type")]
    pub type_name: Option<String>,
    // power
    pub power: Option<f32>,
    // navigation
    pub events: Option<Vec<NavEventItem>>,
    pub results: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NavEventItem {
    pub map: Option<String>,
    pub path: Option<String>,
    pub goal_index: Option<i64>,
    #[serde(rename = "type")]
    pub type_name: String,
    pub duration: i64,
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhyAddrInfo {
    pub city: String,
    pub metro: i64,
    pub area: i64,
    pub country: String,
    pub region: String,
    pub range: [i64; 2],
    pub ip: String,
    pub eu: String,
    pub timezone: String,
    pub ll: [f32; 2]
}

pub fn simple_response<'a>(content: String, content_type: ContentType, status: Status) -> Response<'a> {
    let length = content.len();
    Response::build().raw_body(Body::Sized(Cursor::new(content), length as u64))
        .header_adjoin(content_type)
        .status(status)
        .finalize()
}