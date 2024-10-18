use crate::id::{AlkaneId};
use anyhow::{Result};
use metashrew_support::utils::{consume_sized_int};
use protorune_support::rune_transfer::{RuneTransfer};

#[derive(Default, Clone, Debug)]
pub struct AlkaneTransfer {
    pub id: AlkaneId,
    pub value: u128,
}

impl From<Vec<RuneTransfer>> for AlkaneTransferParcel {
    fn from(v: Vec<RuneTransfer>) -> AlkaneTransferParcel {
        AlkaneTransferParcel(
            v.into_iter()
                .map(|incoming| AlkaneTransfer {
                    id: incoming.id.into(),
                    value: incoming.value,
                })
                .collect(),
        )
    }
}

impl Into<RuneTransfer> for AlkaneTransfer {
  fn into(self) -> RuneTransfer {
    RuneTransfer {
      id: self.id.into(),
      value: self.value
    }
  }
}

impl Into<Vec<RuneTransfer>> for AlkaneTransferParcel {
  fn into(self) -> Vec<RuneTransfer> {
    self.0.into_iter().map(|v| v.into()).collect()
  }
}

#[derive(Default, Clone, Debug)]
pub struct AlkaneTransferParcel(pub Vec<AlkaneTransfer>);

impl AlkaneTransferParcel {
    pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<AlkaneTransferParcel> {
        let mut result = AlkaneTransferParcel::default();
        for _i in [0..consume_sized_int::<u128>(cursor)?] {
            result.0.push(AlkaneTransfer::parse(cursor)?);
        }
        Ok(result)
    }
}

impl AlkaneTransfer {
    pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<AlkaneTransfer> {
        let id = AlkaneId::parse(cursor)?;
        let value = consume_sized_int::<u128>(cursor)?;
        Ok(AlkaneTransfer { id, value })
    }
}
