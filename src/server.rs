use rocket::{get, post, routes, Rocket};
use rocket::http::{Status, ContentType};
use rocket_contrib::json::Json;
use rocket::response::Response;
use super::models::{simple_response, TimeStampQuery, Record, ServerRecordItem, PhyAddrInfo};
use serde_json::json;
use super::database::DATABASE_POOL;
use mongodb::bson::{Document, Bson};
use std::collections::HashMap;
use super::req_guards::IpAddr;
use cached::proc_macro::cached;

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
        }).to_string(), ContentType::JavaScript, Status::BadRequest);
    }

    let queryres = DATABASE_POOL.lock().unwrap().lock()
        .lock().unwrap().get_timestamp(timequery);
    if let Err(_) = queryres {
        // 未找到记录则返回空数组
        return simple_response(json!({
            "timestamp": 1593668491541i64,
        }).to_string(),ContentType::JavaScript, Status::Ok);
    }
    let doc:Document = queryres.unwrap();
    return simple_response(serde_json::to_string(&doc).unwrap(), ContentType::JavaScript, Status::Ok);
}

#[post("/", format = "json", data = "<records>")]
fn upload_records<'a>(ip: IpAddr, records: Json<Vec<Record>>) -> Response<'a> {
    let mut timestamps: HashMap<&str, f64> = HashMap::new();
    let mut insert_records: HashMap<&str, Vec<ServerRecordItem>> = HashMap::new();

    for record in records.iter() {
        // 查找到最新的时间戳
        if !timestamps.contains_key(record.collection.as_str()) {
            let query_res = DATABASE_POOL.lock().unwrap().lock()
                .lock().unwrap().get_timestamp(TimeStampQuery {
                    collection: record.collection.to_owned(),
                    id: record.record.id.to_owned()
                });
            if let Ok(db_record) = query_res {
                let timestamp_record = db_record.get("timestamp");
                let mut timestamp_value = 0f64;
                if timestamp_record.and_then(Bson::as_i64).is_some() {
                    timestamp_value = timestamp_record.and_then(Bson::as_i64).unwrap() as f64;
                }
                if timestamp_record.and_then(Bson::as_f64).is_some() {
                    timestamp_value = timestamp_record.and_then(Bson::as_f64).unwrap();
                }
                timestamps.insert(record.collection.as_str(), timestamp_value);
            }
        }

        if timestamps.contains_key(record.collection.as_str()) && timestamps[record.collection.as_str()] > record.timestamp {
            // 当前数据老于数据库数据
            continue;
        }
        if !insert_records.contains_key(record.collection.as_str()) {
            insert_records.insert(record.collection.as_str(), Vec::new());
        }

        if let Some(vec_record)  = insert_records.get_mut(record.collection.as_str()) {
            vec_record.push(ServerRecordItem {
                codename: record.record.codename.to_owned(),
                version: record.record.version.to_owned(),
                id: record.record.id.to_owned(),
                timestamp: record.timestamp,
                ip: ip.to_string(),
                location: get_phy_addr(ip.to_string()),
                // exceptions
                info: record.record.info.to_owned(),
                type_name: record.record.type_name.to_owned(),
                // power
                power: record.record.power.to_owned(),
                // navigation
                events: record.record.events.to_owned(),
                results: record.record.results.to_owned(),
            });
        }
    }
    let mut inserted_records = Vec::new();
    // 插入数据库
    for vec_record in insert_records.iter() {
        if let Ok(_) = DATABASE_POOL.lock().unwrap().lock()
            .lock().unwrap().insert(vec_record.0, vec_record.1) {
                inserted_records.extend(vec_record.1.iter())
            }
    }
    
    return simple_response(serde_json::to_string(&inserted_records).unwrap(), ContentType::JavaScript, Status::Ok);
}

#[cached]
pub fn get_phy_addr(ip: String) -> Option<PhyAddrInfo> {
    let res = reqwest::blocking::Client::new().post("http://xiaoqiang.bwbot.org/ips")
        .form(&[("ip", ip)])
        .send().unwrap().json::<Vec<PhyAddrInfo>>();
    if let Ok(ip_info) = res {
        return ip_info.into_iter().nth(0);
    }
    None
}

pub fn init_server() -> Rocket {
    rocket::ignite().mount("/", routes![
        timestamp,
        upload_records
    ])
}