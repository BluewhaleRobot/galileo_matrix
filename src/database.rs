use lazy_static::lazy_static;
use super::models::{TimeStampQuery,ServerRecord};

use mongodb::{
    bson::{doc, Bson, Document, to_bson},
    sync::{Client},
    options::FindOneOptions,
};

use std::collections::HashMap;

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

    pub fn insert(&self, collection: &str, records: Vec<ServerRecord>) -> Result<String, String> {
        let res = Document::from(records[0]);
        let bsons:Vec<Document> = records.iter().map(Document::from).map(|a| a.unwrap()).collect();
        let res = self.db.collection(collection).insert_many(records, None);

        Ok(String::from(""))
    }
}
use std::ops::{Deref, DerefMut};
// impl Deref for Database  {
//     type Target = Database;

//     fn deref(&self) -> &Self::Target {
//         &self
//     }
// }

// impl DerefMut for Database {
//     type Target = Database;
//     fn deref_mut(&mut self) -> &mut Database {
//         &mut self
//     }
// }

use std::sync::Mutex;

lazy_static! {
    pub static ref database: Mutex<Database> = Mutex::new(Database::new());
}

