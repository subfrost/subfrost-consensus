pub mod proto;

use protobuf::{MessageField, SpecialFields};
use alkanes_support::{parcel::{AlkaneTransfer}, id::{AlkaneId}};
use protorune_support::balance_sheet::{ProtoruneRuneId};

impl From<proto::subfrost::Uint128> for u128 {
    fn from(v: proto::subfrost::Uint128) -> u128 {
        let mut result: Vec<u8> = Vec::<u8>::with_capacity(16);
        result.extend(&v.lo.to_le_bytes());
        result.extend(&v.hi.to_le_bytes());
        let bytes_ref: &[u8] = &result;
        u128::from_le_bytes(bytes_ref.try_into().unwrap())
    }
}

impl From<u128> for proto::subfrost::Uint128 {
    fn from(v: u128) -> proto::subfrost::Uint128 {
        let bytes = v.to_le_bytes().to_vec();
        let mut container: proto::subfrost::Uint128 = proto::subfrost::Uint128::new();
        container.lo = u64::from_le_bytes((&bytes[0..8]).try_into().unwrap());
        container.hi = u64::from_le_bytes((&bytes[8..16]).try_into().unwrap());
        container
    }
}

impl Into<proto::subfrost::AlkaneId> for AlkaneId {
    fn into(self) -> proto::subfrost::AlkaneId {
        proto::subfrost::AlkaneId {
            special_fields: SpecialFields::new(),
            block: MessageField::some(self.block.into()),
            tx: MessageField::some(self.tx.into()),
        }
    }
}

impl Into<proto::subfrost::AlkaneTransfer> for AlkaneTransfer {
    fn into(self) -> proto::subfrost::AlkaneTransfer {
        let mut result = proto::subfrost::AlkaneTransfer::new();
        result.id = MessageField::some(self.id.into());
        result.value = MessageField::some(self.value.into());
        result
    }
}

impl Into<AlkaneId> for proto::subfrost::AlkaneId {
    fn into(self) -> AlkaneId {
        AlkaneId {
            block: self.block.into_option().unwrap().into(),
            tx: self.tx.into_option().unwrap().into(),
        }
    }
}

impl Into<ProtoruneRuneId> for proto::subfrost::AlkaneId {
    fn into(self) -> ProtoruneRuneId {
        ProtoruneRuneId {
            block: self.tx.as_ref().unwrap().clone().into(),
            tx: self.block.as_ref().unwrap().clone().into(),
        }
    }
}
