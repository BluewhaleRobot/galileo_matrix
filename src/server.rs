use rocket::{get, post, routes, Rocket, Outcome::*, Outcome};
use rocket::http::{Status, ContentType};
use rocket_contrib::json::{Json, JsonValue};
use rocket::response::Response;
use super::models::{simple_response, TimeStampQuery, Record, ServerRecord, ServerRecordItem};
use serde_json::json;
use super::database::database;
use mongodb::bson::{Document, Bson};
use std::collections::HashMap;
use super::req_guards::IpAddr;

#[get("/?<id>&<collection>")]
fn timestamp<'a>(id: String, collection:String) -> Response<'a> {
    let timequery = TimeStampQuery{
        id: id,
        collection: collection
    };
    if !timequery.is_valid() {
        return simple_response(json!({
            "status": "error",
            "description": "invalid robot id",
        }).to_string(), ContentType::JavaScript, Status::Ok);
    }

    let queryres = database.lock().unwrap().get_timestamp(timequery);
    if let Err(_) = queryres {
        // 未找到记录则返回空数组
        return simple_response("null".to_string(),ContentType::JavaScript, Status::Ok);
    }
    let doc:Document = queryres.unwrap();
    return simple_response(doc.to_string(), ContentType::HTML, Status::BadRequest);
}

#[post("/", format = "json", data = "<records>")]
fn upload_records<'a>(ip: IpAddr, records: Json<Vec<Record>>) -> Response<'a> {
    let mut timestamps: HashMap<&str, u64> = HashMap::new();
    let mut insert_records:Vec<ServerRecord> = Vec::new();

    for record in records.iter() {
        // 查找到最新的时间戳
        if !timestamps.contains_key(record.collection.as_str()) {
            let query_res = database.lock().unwrap().get_timestamp(TimeStampQuery {
                collection: record.collection.to_owned(),
                id: record.record.id.to_owned()
            });
            if let Ok(db_record) = query_res {
                timestamps.insert(record.collection.as_str(), db_record.get("timestamp").and_then(Bson::as_i64).unwrap() as u64);
            }
        }

        if timestamps.contains_key(record.collection.as_str()) && timestamps[record.collection.as_str()] > record.timestamp {
            // 当前数据老于数据库数据
            continue;
        }
        insert_records.push(ServerRecord {
            timestamp: record.timestamp,
            collection: record.collection.to_owned(),
            record: ServerRecordItem {
                info: record.record.info.to_owned(),
                codename: record.record.codename.to_owned(),
                version: record.record.version.to_owned(),
                type_name: record.record.type_name.to_owned(),
                id: record.record.id.to_owned(),
                timestamp: record.timestamp,
                ip: ip.to_string(),
                location: get_phy_addr(ip.to_string()),
            }
        })
        
    }
    return simple_response("OK".to_string(), ContentType::JavaScript, Status::BadRequest);
}

fn get_phy_addr(ip: String) -> String {
    String::from("")
}

pub fn init_server() -> Rocket {
    rocket::ignite().mount("/", routes![
        timestamp,
        upload_records
    ])
}