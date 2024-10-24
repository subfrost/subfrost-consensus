use crate::runtime::{AlkaneResponder};
use alkanes_support::response::{CallResponse};
use metashrew_support::index_pointer::{KeyValuePointer};

struct StorageHandle(());

impl AlkaneResponder for StorageHandle {
  fn execute() -> CallResponse {
    CallResponse::default()
  }
}

const runtime_storage: StorageHandle = StorageHandle(());

#[derive(Debug, Clone, Default)]
pub struct StoragePointer(pub Arc<Vec<u8>>);


#[allow(dead_code)]
impl KeyValuePointer for StoragePointer {
    fn wrap(word: &Vec<u8>) -> StoragePointer {
        StoragePointer(Arc::<Vec<u8>>::new(word.clone()))
    }
    fn unwrap(&self) -> Arc<Vec<u8>> {
        self.0.clone()
    }
    fn inherits(&mut self, _v: &Self) {}
    fn set(&mut self, v: Arc<Vec<u8>>) {
        runtime_storage.store(self.unwrap(), v.as_ref().clone())
    }
    fn get(&self) -> Arc<Vec<u8>> {
        runtime_storage.load(self.unwrap().as_ref().clone())
    }
}
