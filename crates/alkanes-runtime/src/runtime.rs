use crate::imports::{
  abort,
  __load_storage,
  __request_storage,
  __log,
  __balance,
  __request_context,
  __load_context,
  __sequence,
  __fuel,
  __returndatacopy,
  __request_transaction,
  __load_transaction,
  __request_block,
  __load_block,
  __call,
  __staticcall,
  __delegatecall
};

use alkanes_support::storage::{StorageMap};

static mut _CACHE: Option<StorageMap> = None;

pub trait AlkaneResponder {
  fn context(&self) -> Context {
    let buffer: Vec<u8> = to_arraybuffer_layout(vec![0; __request_context() as usize]);
    __load_context(to_ptr(&mut buffer) + 4);
    Context::parse((&mut Cursor::<Vec<u8>>::new((&buffer[4..]).to_vec())))
  }
  fn block(&self) -> Vec<u8> {
    let buffer: Vec<u8> = to_arraybuffer_layout(vec![0; __request_block() as usize]);
    __load_block(to_ptr(&mut buffer) + 4);
    (&buffer[4..]).to_vec()
    
  }
  fn initialize(&self) {
    unsafe {
      _CACHE = Some(StorageMap::default());
    }
  }
  fn transaction(&self) -> Vec<u8> {
    let buffer: Vec<u8> = to_arraybuffer_layout(vec![0; __request_transaction() as usize]);
    __load_transaction(to_ptr(&mut buffer) + 4);
    (&buffer[4..]).to_vec()

  }
  fn load(&self, k: Vec<u8>) -> Vec<u8> {
    let key_bytes = to_arraybuffer_layout(&k);
    let key = to_ptr(&mut key_bytes) + 4;
    let buffer: Vec<u8> = to_arraybuffer_layout(vec![0; __request_storage(key) as usize]);
    __load_storage(key, to_ptr(&mut buffer) + 4);
    (&buffer[4..]).to_vec()
  }
  fn store(&self, k: Vec<u8>, v: Vec<u8>) {
    _CACHE.unwrap().set(&k, &v);
  }
  fn balance(&self, who: &AlkaneId, what: &AlkaneId) -> u128 {
    let who_bytes: Vec<u8> = to_arraybuffer_layout::<Vec<u8>>(who.clone().into());
    let what_bytes: Vec<u8> = to_arraybuffer_layout::<Vec<u8>>(what.clone().into());
    let who_ptr = to_ptr(&mut who_bytes) + 4;
    let what_ptr = to_ptr(&mut what_bytes) + 4;
    let output: Vec<u8> = to_arraybuffer_layout::<Vec<u8>>(vec![0u8; 16]);
    __balance(who_ptr, what_ptr, to_ptr(&mut output) + 4);
    u128::from_le_bytes((&output[4..]).try_into().unwrap())
  }
  fn sequence(&self) -> u128 {
    let buffer: Vec<u8> = to_arraybuffer_layout(vec![0; 16]);
    __sequence(to_ptr(&mut buffer) + 4);
    u64::from_le_bytes((&buffer[4..]).try_into().unwrap())
  }
  fn execute(&self, runtime: &AlkaneRuntime) -> CallResponse;
}
