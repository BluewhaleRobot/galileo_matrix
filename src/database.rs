use lazy_static::lazy_static;
use std::sync::Mutex;
use std::{thread, time};
use super::models::{TimeStampQuery, ServerRecordItem};
use mongodb::{bson::doc, options::FindOneOptions, sync::Client};
use mongodb::bson::{Document, to_bson, to_document};

pub struct Database {
    collections: Vec<String>
}

lazy_static! {
    static ref CLIENT:Client = Client::with_uri_str("mongodb://127.0.0.1:22076").expect("connect to database failed");
}

impl Database {
    pub fn new() -> Database {
        Database {
            collections: Vec::new()
        }
    }

    fn get_connection(&self) -> mongodb::sync::Database {
        return CLIENT.database("galileo_matrix");
    }

    pub fn has_collection(&mut self, collection_name: &str) -> bool {
        if self.collections.iter().any(|c| c == collection_name){
            return true;
        }
        self.collections = self.get_connection().list_collection_names(None).expect("list collection failed");
        if self.collections.iter().any(|c| c == collection_name){
            return true;
        }
        return false;
    }

    pub fn get_timestamp(&mut self, query: TimeStampQuery) -> Result<Document, String>{
        if !self.has_collection(query.collection.as_str()) {
            return Err(format!("Collection {} not found", &query.collection));
        }
        let collection = self.get_connection().collection(query.collection.as_str());
        let find_options = FindOneOptions::builder().sort(doc! { "timestamp": -1 }).build();
        let result = collection.find_one(doc! {"id": query.id}, find_options);
        if let Err(_) = result {
            return Err("record not found".to_string());
        }
        if let Ok(record) = result {
            if let Some(document) = record{
                return Ok(document);
            }
        }
        return Err("record not found".to_string());
    }

    pub fn insert(&self, collection: &str, records: &Vec<ServerRecordItem>) -> Result<String, String> {
        let bsons:Vec<Document> = records.iter().map(|d| {
             to_document(d).unwrap()
        }).collect();
        let res = self.get_connection().collection(collection).insert_many(bsons, None);
        if let Err(_) = res {
            return Err("insert data to database failed".to_string());
        }
        Ok(String::from(""))
    }
}

lazy_static! {
    pub static ref DATABASE_POOL: Mutex<Database> = Mutex::new(
        Database::new()
    );
}

