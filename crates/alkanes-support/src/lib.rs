pub mod cellpack;
pub mod constants;
pub mod context;
pub mod envelope;
pub mod gz;
pub mod id;
pub mod parcel;
pub mod response;
pub mod storage;
pub mod utils;
pub mod proto;
pub mod witness;

use crate::response::{ExtendedCallResponse};
use crate::parcel::{AlkaneTransfer};
use crate::id::{AlkaneId};
use protobuf::{SpecialFields, MessageField};
use protorune_support::balance_sheet::ProtoruneRuneId;

impl From<proto::alkanes::Uint128> for u128 {
  fn from(v: proto::alkanes::Uint128) -> u128 {
    let mut result: Vec<u8> = Vec::<u8>::with_capacity(16);
    result.extend(&v.lo.to_le_bytes());
    result.extend(&v.hi.to_le_bytes());
    let bytes_ref: &[u8] = &result;
    u128::from_le_bytes(bytes_ref.try_into().unwrap())
  }
}

impl From<u128> for proto::alkanes::Uint128 {
  fn from(v: u128) -> proto::alkanes::Uint128 {
    let bytes = v.to_le_bytes().to_vec();
    let mut container: proto::alkanes::Uint128 = proto::alkanes::Uint128::new();
    container.lo = u64::from_le_bytes((&bytes[0..8]).try_into().unwrap());
    container.hi = u64::from_le_bytes((&bytes[8..16]).try_into().unwrap());
    container
  }
}

impl Into<proto::alkanes::AlkaneId> for AlkaneId {
    fn into(self) -> proto::alkanes::AlkaneId {
        proto::alkanes::AlkaneId {
            special_fields: SpecialFields::new(),
            block: MessageField::some(self.block.into()),
            tx: MessageField::some(self.tx.into())
        }
    }
}

impl Into<proto::alkanes::AlkaneTransfer> for AlkaneTransfer {
    fn into(self) -> proto::alkanes::AlkaneTransfer {
        let mut result = proto::alkanes::AlkaneTransfer::new();
        result.id = MessageField::some(self.id.into());
        result.value = MessageField::some(self.value.into());
        result
    }
}

impl Into<proto::alkanes::ExtendedCallResponse> for ExtendedCallResponse {
    fn into(self) -> proto::alkanes::ExtendedCallResponse {
        let mut result: proto::alkanes::ExtendedCallResponse =
            proto::alkanes::ExtendedCallResponse::new();
        result.storage = self
            .storage
            .0
            .into_iter()
            .map(|(key, value)| proto::alkanes::KeyValuePair {
                key,
                value,
                special_fields: SpecialFields::new(),
            })
            .collect::<Vec<proto::alkanes::KeyValuePair>>();
        result.data = self.data;
        result.alkanes = self
            .alkanes
            .0
            .into_iter()
            .map(|v| proto::alkanes::AlkaneTransfer {
                id: MessageField::some(proto::alkanes::AlkaneId {
                    block: MessageField::some(v.id.block.into()),
                    tx: MessageField::some(v.id.tx.into()),
                    special_fields: SpecialFields::new(),
                }),
                special_fields: SpecialFields::new(),
                value: MessageField::some(v.value.into())
            })
            .collect::<Vec<proto::alkanes::AlkaneTransfer>>();

        result
    }
}

impl Into<AlkaneId> for proto::alkanes::AlkaneId {
    fn into(self) -> AlkaneId {
        AlkaneId {
            block: self.block.into_option().unwrap().into(),
            tx: self.tx.into_option().unwrap().into(),
        }
    }
}

impl Into<ProtoruneRuneId> for proto::alkanes::AlkaneId {
  fn into(self) -> ProtoruneRuneId {
    ProtoruneRuneId {
      block: self.tx.as_ref().unwrap().clone().into(),
      tx: self.block.as_ref().unwrap().clone().into()
    }
  }
}

impl Into<proto::alkanes::AlkaneInventoryRequest> for AlkaneId {
    fn into(self) -> proto::alkanes::AlkaneInventoryRequest {
        proto::alkanes::AlkaneInventoryRequest {
            id: MessageField::some(proto::alkanes::AlkaneId {
                block: MessageField::some(self.block.into()),
                tx: MessageField::some(self.tx.into()),
                special_fields: SpecialFields::new(),
            }),
            special_fields: SpecialFields::new(),
        }
    }
}
