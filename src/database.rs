use lazy_static::lazy_static;
use std::sync::Mutex;
use super::models::{TimeStampQuery, ServerRecordItem};

use mongodb::{
    bson::{doc, Document, to_bson},
    sync::{Client},
    options::FindOneOptions,
};


pub struct Database {
    collections: Vec<String>,
    db: mongodb::sync::Database,
}

impl Database {
    pub fn new() -> Database {
        let client = Client::with_uri_str("mongodb://127.0.0.1:27017").expect("connect to database failed");
        let database_name = "galileo_matrix";
    
        Database {
            collections: Vec::new(),
            db: client.database(database_name)
        }
    }

    pub fn has_collection(&mut self, collection_name: &str) -> bool {
        if self.collections.iter().any(|c| c == collection_name){
            return true;
        }
        self.collections = self.db.list_collection_names(None).expect("list collection failed");
        if self.collections.iter().any(|c| c == collection_name){
            return true;
        }
        return false;
    }

    pub fn get_timestamp(&mut self, query: TimeStampQuery) -> Result<Document, String>{
        if !self.has_collection(query.collection.as_str()) {
            return Err(format!("Collection {} not found", &query.collection));
        }
        let filter = doc! { "id": query.id };
        let find_options = FindOneOptions::builder().sort(doc! { "timestamp": -1 }).build();
        let collection = self.db.collection(query.collection.as_str());
        let result = collection.find_one(filter, find_options);
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
        let bsons:Vec<Document> = records.iter().map(|d| -> Document {
            to_bson(d).unwrap().as_document().unwrap().to_owned()
        }).collect();
        let res = self.db.collection(collection).insert_many(bsons, None);
        if let Err(_) = res {
            return Err("insert data to database failed".to_string());
        }
        Ok(String::from(""))
    }
}

lazy_static! {
    pub static ref DATABASE: Mutex<Database> = Mutex::new(Database::new());
}

