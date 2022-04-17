use storage::store::{DbError, DbStore};
use std::sync::Arc;
use std::sync::RwLock;
use protos::fmq_proto::{ SendMessageRequest, SendMessageResponse };
use tonic::{Request, Response, Status};
use uuid::Uuid;
use prost::Message;

pub struct Store {
    message_store: Box<dyn DbStore>,
    index_store: Box<dyn DbStore>,
}

pub struct Queue {
    //Want to lock both stores when operations
    store: Arc<RwLock<Store>>,
    name: String,
}

impl Queue {
    pub fn new(name: &str, message_store: Box<dyn DbStore>, index_store: Box<dyn DbStore>) -> Queue {
        Queue {
            store: Arc::new(RwLock::new(Store {
                message_store,
                index_store
            })),
            name: name.to_string()
        }
    }

    pub fn enqueue(&self, payload: Vec<u8>) -> Result<String, DbError>{
        let message_id = Uuid::new_v4().to_string();
        let store = self.store.write().unwrap();
        store.message_store.set(message_id.as_bytes().to_vec().as_ref(), payload)?;
        Ok(message_id)
    }
}