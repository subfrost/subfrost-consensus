use super::{
    get_memory, read_arraybuffer, run, send_to_arraybuffer, sequence_pointer, AlkanesState,
    Extcall, Saveable, SaveableExtendedCallResponse,
};
use crate::utils::{pipe_storagemap_to, transfer_from};
use alkanes_support::{
    cellpack::Cellpack, id::AlkaneId, parcel::AlkaneTransferParcel, response::CallResponse,
    storage::StorageMap,
};
use anyhow::Result;
use metashrew::index_pointer::IndexPointer;
use metashrew::{
    print,
    stdio::{stdout, Write},
};
use metashrew_support::index_pointer::KeyValuePointer;

use protorune_support::utils::consensus_encode;
use std::io::Cursor;
use wasmi::*;

pub struct AlkanesHostFunctionsImpl(());
impl AlkanesHostFunctionsImpl {
    pub(super) fn _abort<'a>(caller: Caller<'_, AlkanesState>) {
        AlkanesHostFunctionsImpl::abort(caller, 0, 0, 0, 0);
    }
    pub(super) fn abort<'a>(mut caller: Caller<'_, AlkanesState>, _: i32, _: i32, _: i32, _: i32) {
        caller.data_mut().had_failure = true;
    }
    pub(super) fn request_storage<'a>(
        caller: &mut Caller<'_, AlkanesState>,
        k: i32,
    ) -> Result<i32> {
        let mem = get_memory(caller)?;
        let key = {
            let data = mem.data(&caller);
            read_arraybuffer(data, k)?
        };
        let myself = caller.data_mut().context.lock().unwrap().myself.clone();
        Ok(caller
            .data_mut()
            .context
            .lock()
            .unwrap()
            .message
            .atomic
            .keyword("/alkanes/")
            .select(&myself.into())
            .keyword("/storage")
            .select(&key)
            .get()
            .len()
            .try_into()?)
    }
    pub(super) fn load_storage<'a>(
        caller: &mut Caller<'_, AlkanesState>,
        k: i32,
        v: i32,
    ) -> Result<i32> {
        let mem = get_memory(caller)?;
        let key = {
            let data = mem.data(&caller);
            read_arraybuffer(data, k)?
        };
        let value = {
            let myself = caller.data_mut().context.lock().unwrap().myself.clone();
            (&caller.data_mut().context.lock().unwrap().message)
                .atomic
                .keyword("/alkanes/")
                .select(&myself.into())
                .keyword("/storage")
                .select(&key)
                .get()
        };
        send_to_arraybuffer(caller, v.try_into()?, value.as_ref())
    }
    pub(super) fn request_context(caller: &mut Caller<'_, AlkanesState>) -> Result<i32> {
        Ok(caller
            .data_mut()
            .context
            .lock()
            .unwrap()
            .serialize()
            .len()
            .try_into()?)
    }
    pub(super) fn load_context(caller: &mut Caller<'_, AlkanesState>, v: i32) -> Result<i32> {
        let context = caller.data_mut().context.lock().unwrap().serialize();
        send_to_arraybuffer(caller, v.try_into()?, &context)
    }
    pub(super) fn request_transaction(caller: &mut Caller<'_, AlkanesState>) -> Result<i32> {
        Ok(consensus_encode(
            &caller
                .data_mut()
                .context
                .lock()
                .unwrap()
                .message
                .transaction,
        )?
        .len()
        .try_into()?)
    }
    pub(super) fn returndatacopy(caller: &mut Caller<'_, AlkanesState>, output: i32) -> Result<()> {
        let context = caller.data_mut().context.lock().unwrap().returndata.clone();
        send_to_arraybuffer(caller, output.try_into()?, &context)?;
        Ok(())
    }
    pub(super) fn load_transaction(caller: &mut Caller<'_, AlkanesState>, v: i32) -> Result<()> {
        let context = consensus_encode(
            &caller
                .data_mut()
                .context
                .lock()
                .unwrap()
                .message
                .transaction,
        )?;
        send_to_arraybuffer(caller, v.try_into()?, &context)?;
        Ok(())
    }
    pub(super) fn request_block(caller: &mut Caller<'_, AlkanesState>) -> Result<i32> {
        Ok(
            consensus_encode(&caller.data_mut().context.lock().unwrap().message.block)?
                .len()
                .try_into()?,
        )
    }
    pub(super) fn load_block(caller: &mut Caller<'_, AlkanesState>, v: i32) -> Result<()> {
        let context = consensus_encode(&caller.data_mut().context.lock().unwrap().message.block)?;
        send_to_arraybuffer(caller, v.try_into()?, &context)?;
        Ok(())
    }
    pub(super) fn sequence(caller: &mut Caller<'_, AlkanesState>, output: i32) -> Result<()> {
        let buffer: Vec<u8> =
            (&sequence_pointer(&caller.data_mut().context.lock().unwrap().message.atomic)
                .get_value::<u128>()
                .to_le_bytes())
                .to_vec();
        send_to_arraybuffer(caller, output.try_into()?, &buffer)?;
        Ok(())
    }
    pub(super) fn fuel(caller: &mut Caller<'_, AlkanesState>, output: i32) -> Result<()> {
        let buffer: Vec<u8> = (&caller.get_fuel().unwrap().to_le_bytes()).to_vec();
        send_to_arraybuffer(caller, output.try_into()?, &buffer)?;
        Ok(())
    }
    pub(super) fn balance<'a>(
        caller: &mut Caller<'a, AlkanesState>,
        who_ptr: i32,
        what_ptr: i32,
        output: i32,
    ) -> Result<()> {
        let (who, what) = {
            let mem = get_memory(caller)?;
            let data = mem.data(&caller);
            (
                AlkaneId::parse(&mut Cursor::new(read_arraybuffer(data, who_ptr)?))?,
                AlkaneId::parse(&mut Cursor::new(read_arraybuffer(data, what_ptr)?))?,
            )
        };
        let balance = caller
            .data_mut()
            .context
            .lock()
            .unwrap()
            .message
            .atomic
            .keyword("/alkanes/")
            .select(&who.into())
            .keyword("/balances/")
            .select(&what.into())
            .get()
            .as_ref()
            .clone();
        send_to_arraybuffer(caller, output.try_into()?, &balance)?;
        Ok(())
    }
    pub(super) fn extcall<'a, T: Extcall>(
        caller: &mut Caller<'_, AlkanesState>,
        cellpack_ptr: i32,
        incoming_alkanes_ptr: i32,
        checkpoint_ptr: i32,
        start_fuel: u64,
    ) -> Result<i32> {
        let mem = get_memory(caller)?;
        let data = mem.data(&caller);
        let buffer = read_arraybuffer(data, cellpack_ptr)?;
        let cellpack = Cellpack::parse(&mut Cursor::new(buffer))?;
        let buf = read_arraybuffer(data, incoming_alkanes_ptr)?;
        let incoming_alkanes = AlkaneTransferParcel::parse(&mut Cursor::new(buf))?;
        let storage_map =
            StorageMap::parse(&mut Cursor::new(read_arraybuffer(data, checkpoint_ptr)?))?;
        let subcontext = {
            let mut context = caller.data_mut().context.lock().unwrap();
            context.message.atomic.checkpoint();
            pipe_storagemap_to(
                &storage_map,
                &mut context.message.atomic.derive(
                    &IndexPointer::from_keyword("/alkanes/").select(&context.myself.into()),
                ),
            );
            if let Err(_) = transfer_from(
                &incoming_alkanes,
                &mut context.message.atomic.derive(&IndexPointer::default()),
                &context.myself,
                &cellpack.target,
            ) {
                context.message.atomic.rollback();
                context.returndata = Vec::<u8>::new();
                return Ok(0);
            }
            let mut subbed = (&*context).clone();
            subbed.message.atomic = context.message.atomic.derive(&IndexPointer::default());
            (subbed.caller, subbed.myself) = T::change_context(
                cellpack.target.clone(),
                context.caller.clone(),
                context.myself.clone(),
            );
            subbed.returndata = vec![];
            subbed.incoming_alkanes = incoming_alkanes.clone();
            subbed.inputs = cellpack.inputs.clone();
            subbed
        };
        match run(subcontext.clone(), &cellpack, start_fuel, T::isdelegate()) {
            Ok(response) => {
                let mut context = caller.data_mut().context.lock().unwrap();
                let mut saveable: SaveableExtendedCallResponse = response.clone().into();
                saveable.associate(&subcontext);
                saveable.save(&mut context.message.atomic)?;
                T::handle_atomic(&mut context.message.atomic);
                let plain_response: CallResponse = response.into();
                let serialized = plain_response.serialize();
                context.returndata = serialized;
                Ok(context.returndata.len().try_into()?)
            }
            Err(_) => {
                let mut context = caller.data_mut().context.lock().unwrap();
                context.message.atomic.rollback();
                context.returndata = vec![];
                Ok(0)
            }
        }
    }
    pub(super) fn log<'a>(caller: &mut Caller<'_, AlkanesState>, v: i32) -> Result<()> {
        let mem = get_memory(caller)?;
        let message = {
            let data = mem.data(&caller);
            read_arraybuffer(data, v)?
        };
        print!("{}", String::from_utf8(message)?);
        Ok(())
    }
}
