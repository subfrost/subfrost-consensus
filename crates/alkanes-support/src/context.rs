use crate::{
    id::AlkaneId,
    parcel::{AlkaneTransfer, AlkaneTransferParcel},
};
use anyhow::Result;
use metashrew_support::utils::{consume_sized_int, is_empty};
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Context {
    pub myself: AlkaneId,
    pub caller: AlkaneId,
    pub incoming_alkanes: AlkaneTransferParcel,
    pub inputs: Vec<u128>,
}

use crate::{
    println,
    stdio::{stdout, Write},
};
// impl AlkaneTransferParcel {
//     pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<AlkaneTransferParcel> {
//         let mut result = AlkaneTransferParcel::default();
//         println!("stuff");
//         for _i in [0..consume_sized_int::<u128>(cursor)?] {
//             result.0.push(AlkaneTransfer::parse(cursor)?);
//         }
//         Ok(result)
//     }
// }
impl Context {
    pub fn parse(v: &mut Cursor<Vec<u8>>) -> Result<Context> {
        let mut result = Context::default();
        result.myself = AlkaneId::parse(v)?;
        result.caller = AlkaneId::parse(v)?;
        let mut res = AlkaneTransferParcel::default();
        let len = consume_sized_int::<u128>(v)?;
        println!("{}", len);
        for i in 0..len {
            println!("{}", i);
            let id = AlkaneId::parse(v)?;
            println!("parsed id");
            let value = consume_sized_int::<u128>(v)?;
            println!("parsed value");
            res.0.push(AlkaneTransfer { id, value })
        }
        // result.incoming_alkanes = parse(v)?;
        while !is_empty(v) {
            result.inputs.push(consume_sized_int::<u128>(v)?);
        }
        Ok(result)
    }
}
