use storage::store::open_store;
use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
use protos::fmq_proto::queue_service_server::QueueService;
use crate::queue::Queue;
use tonic::{Response, Request, Status, Code};
use protos::fmq_proto::{SendMessageRequest, SendMessageResponse, ReceiveMessageRequest, ReceiveMessageResponse,
                        DeleteMessageRequest, DeleteMessageResponse, ListQueuesRequest, ListQueuesResponse,
                        CreateQueueRequest, CreateQueueResponse, DeleteQueueRequest, DeleteQueueResponse};
use prost::Message;

pub struct QueueManager {
    active_queues: Arc<RwLock<HashMap<String, Queue>>>,
    storage_dir: String,
}

impl QueueManager {

    fn new(store_dir: &str) -> QueueManager {
        QueueManager {
            active_queues: Arc::new(RwLock::new(HashMap::new())),
            storage_dir: store_dir.to_string(),
        }
    }

    fn activate_queue(
        &self,
        queue_name: &str
    ) {
        let mut queue_map = self.active_queues.write().unwrap();
        if !queue_map.contains_key(queue_name) {
            queue_map.insert(queue_name.to_string(), self.open_queue(queue_name));
        }
    }

    fn open_queue(&self, queue_name: &str) -> Queue {
        let message_store = open_store(format!("{}/{}",
                                               self.storage_dir.as_str(), queue_name).as_str()).unwrap();
        let index_store = open_store(format!("{}/{}_index",
                                             self.storage_dir.as_str(), queue_name).as_str()).unwrap();
        Queue::new(queue_name, message_store, index_store)
    }
}

#[tonic::async_trait]
impl QueueService for QueueManager {

    async fn send_message(&self, request: Request<SendMessageRequest>) -> Result<Response<SendMessageResponse>, Status> {
        let mut value_buf = Vec::<u8>::with_capacity(200);
        let _r = request.get_ref().encode(&mut value_buf);
        let queue_name = request.get_ref().queue_name.clone();
        self.activate_queue(queue_name.as_str());
        let active_queues_map = self.active_queues.read().unwrap();
        let result = active_queues_map.get(queue_name.as_str()).unwrap().enqueue(value_buf);
        match result {
            Ok(message_id) => Ok(Response::new(SendMessageResponse{
                message_id
            })),
            Err(err) => Err(Status::new(Code::Unknown, "Failed to send message!"))
        }
    }

    async fn receive_message(&self, request: Request<ReceiveMessageRequest>) -> Result<Response<ReceiveMessageResponse>, Status> {
        todo!()
    }

    async fn delete_message(&self, request: Request<DeleteMessageRequest>) -> Result<Response<DeleteMessageResponse>, Status> {
        todo!()
    }

    async fn list_queues(&self, request: Request<ListQueuesRequest>) -> Result<Response<ListQueuesResponse>, Status> {
        todo!()
    }

    async fn create_queue(&self, request: Request<CreateQueueRequest>) -> Result<Response<CreateQueueResponse>, Status> {
        todo!()
    }

    async fn delete_queue(&self, request: Request<DeleteQueueRequest>) -> Result<Response<DeleteQueueResponse>, Status> {
        todo!()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[tokio::test]
    async fn test_storage_manager() {
        let queue_manager = QueueManager::new("/tmp/fmq");
        queue_manager.send_message(Request::new(SendMessageRequest{
            queue_name: "test".to_string(),
            payload: "value".as_bytes().to_vec(),
            delay: 0,
            metadata: "test".to_string()
        })).await;
    }
}