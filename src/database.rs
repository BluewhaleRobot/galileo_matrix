use lazy_static::lazy_static;
use std::sync::Mutex;
use std::{thread, time};
use super::models::{TimeStampQuery, ServerRecordItem};
use r2d2::PooledConnection;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager, mongodb::db::ThreadedDatabase};
use r2d2_mongodb::mongodb as mongodb;
use mongodb::coll::options::FindOptions;
use r2d2_mongodb::mongodb::{Document, doc, bson, to_bson};

type Pool = r2d2::Pool<MongodbConnectionManager>;
type PooledConn = PooledConnection<MongodbConnectionManager>;

pub struct Database {
    collections: Vec<String>,
    db: Pool
}

impl Database {
    pub fn new() -> Database {
        print!("############## ok1");
        Database {
            collections: Vec::new(),
            db: Pool::builder().max_size(64).build(
                MongodbConnectionManager::new(
                    ConnectionOptions::builder()
                    .with_host("127.0.0.1", 27017)
                    .with_db("galileo_matrix")
                    .build()
                )
            ).expect("connect to database pool failed")
        }
    }

    fn get_connection(&self) -> PooledConn {
        self.db.get().expect("get db connection failed")
    }

    pub fn has_collection(&mut self, collection_name: &str) -> bool {
        if self.collections.iter().any(|c| c == collection_name){
            return true;
        }
        self.collections = self.get_connection().collection_names(None).expect("list collection failed");
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
        let mut find_options = FindOptions::new();
        find_options.sort = Some(doc!{"timestamp": -1});
        let collection = self.get_connection().collection(query.collection.as_str());
        let result = collection.find_one(Some(filter), Some(find_options));
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

