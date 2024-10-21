#[allow(unused_imports)]
use crate::imports::{
    __balance, __call, __delegatecall, __fuel, __load_block, __load_context, __load_storage,
    __load_transaction, __log, __request_block, __request_context, __request_storage,
    __request_transaction, __returndatacopy, __sequence, __staticcall, abort,
};
use crate::{
    println,
    stdio::{stdout, Write},
};
use anyhow::Result;
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use std::io::Cursor;

use alkanes_support::{
    context::Context, id::AlkaneId, response::CallResponse, storage::StorageMap,
};

static mut _CACHE: Option<StorageMap> = None;

pub trait AlkaneResponder {
    fn context(&self) -> Result<Context> {
        println!("getting context");
        unsafe {
            println!("requesting context");
            let mut buffer: Vec<u8> = to_arraybuffer_layout(vec![0; __request_context() as usize]);
            println!("loading context");
            __load_context(to_ptr(&mut buffer) + 4);
            println!("loaded context");
            let res = Context::parse(&mut Cursor::<Vec<u8>>::new((&buffer[4..]).to_vec()));
            println!("{}", res.is_err());
            res
        }
    }
    fn block(&self) -> Vec<u8> {
        unsafe {
            let mut buffer: Vec<u8> = to_arraybuffer_layout(vec![0; __request_block() as usize]);
            __load_block(to_ptr(&mut buffer) + 4);
            (&buffer[4..]).to_vec()
        }
    }
    fn initialize(&self) {
        unsafe {
            _CACHE = Some(StorageMap::default());
        }
    }
    fn transaction(&self) -> Vec<u8> {
        unsafe {
            let mut buffer: Vec<u8> =
                to_arraybuffer_layout(vec![0; __request_transaction() as usize]);
            __load_transaction(to_ptr(&mut buffer) + 4);
            (&buffer[4..]).to_vec()
        }
    }
    fn load(&self, k: Vec<u8>) -> Vec<u8> {
        unsafe {
            let mut key_bytes = to_arraybuffer_layout(&k);
            let key = to_ptr(&mut key_bytes) + 4;
            let mut buffer: Vec<u8> =
                to_arraybuffer_layout(vec![0; __request_storage(key) as usize]);
            __load_storage(key, to_ptr(&mut buffer) + 4);
            (&buffer[4..]).to_vec()
        }
    }
    fn store(&self, k: Vec<u8>, v: Vec<u8>) {
        unsafe {
            _CACHE.as_mut().unwrap().set(&k, &v);
        }
    }
    fn balance(&self, who: &AlkaneId, what: &AlkaneId) -> u128 {
        unsafe {
            let mut who_bytes: Vec<u8> = to_arraybuffer_layout::<Vec<u8>>(who.clone().into());
            let mut what_bytes: Vec<u8> = to_arraybuffer_layout::<Vec<u8>>(what.clone().into());
            let who_ptr = to_ptr(&mut who_bytes) + 4;
            let what_ptr = to_ptr(&mut what_bytes) + 4;
            let mut output: Vec<u8> = to_arraybuffer_layout::<Vec<u8>>(vec![0u8; 16]);
            __balance(who_ptr, what_ptr, to_ptr(&mut output) + 4);
            u128::from_le_bytes((&output[4..]).try_into().unwrap())
        }
    }
    fn sequence(&self) -> u128 {
        unsafe {
            let mut buffer: Vec<u8> = to_arraybuffer_layout(vec![0; 16]);
            __sequence(to_ptr(&mut buffer) + 4);
            u128::from_le_bytes((&buffer[4..]).try_into().unwrap())
        }
    }
    fn execute(&self) -> CallResponse;
}
