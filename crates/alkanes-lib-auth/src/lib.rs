use alkanes_runtime::{runtime::AlkaneResponder, storage::StoragePointer};
use alkanes_support::{
    cellpack::Cellpack,
    id::AlkaneId,
    parcel::{AlkaneTransfer, AlkaneTransferParcel},
};
use anyhow::{anyhow, Result};
use metashrew_support::index_pointer::KeyValuePointer;
use std::sync::Arc;

pub trait AuthenticatedResponder: AlkaneResponder {
    fn deploy_auth_token(&self, units: u128) -> Result<AlkaneTransfer> {
        let cellpack = Cellpack {
            target: AlkaneId {
                block: 3,
                tx: 0xffee,
            },
            inputs: vec![0x0, units],
        };
        let sequence = self.sequence();
        let response = self.call(&cellpack, &AlkaneTransferParcel::default(), self.fuel())?;
        StoragePointer::from_keyword("/auth").set(Arc::new(<AlkaneId as Into<Vec<u8>>>::into(
            AlkaneId {
                block: 2,
                tx: sequence,
            },
        )));
        if response.alkanes.0.len() < 1 {
            Err(anyhow!("auth token not returned with factory"))
        } else {
            Ok(response.alkanes.0[0])
        }
    }
    fn auth_token(&self) -> Result<AlkaneId> {
        let pointer = StoragePointer::from_keyword("/auth");
        Ok(pointer.get().as_ref().clone().try_into()?)
    }
    fn only_owner(&self) -> Result<()> {
        let auth_token = self.auth_token()?;
        let cellpack = Cellpack {
            target: auth_token,
            inputs: vec![0x1],
        };
        let response = self.call(
            &cellpack,
            &AlkaneTransferParcel(vec![AlkaneTransfer {
                id: cellpack.target.clone(),
                value: 1,
            }]),
            self.fuel(),
        )?;
        if response.data == vec![0x01] {
            Ok(())
        } else {
            Err(anyhow!("only_owner: returned error"))
        }
    }
}
