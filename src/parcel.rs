use crate::id::AlkaneId;
use anyhow::{anyhow, Result};
use metashrew::index_pointer::KeyValuePointer;
use metashrew::utils::consume_sized_int;
use protorune::incoming_rune::IncomingRune;

#[derive(Default, Clone)]
pub struct AlkaneTransfer {
    pub id: AlkaneId,
    pub value: u128,
}

impl From<Vec<IncomingRune>> for AlkaneTransferParcel {
    fn from(v: Vec<IncomingRune>) -> AlkaneTransferParcel {
        AlkaneTransferParcel(
            v.into_iter()
                .map(|incoming| AlkaneTransfer {
                    id: incoming.rune.into(),
                    value: incoming.amount,
                })
                .collect(),
        )
    }
}

#[derive(Default, Clone)]
pub struct AlkaneTransferParcel(pub Vec<AlkaneTransfer>);

impl AlkaneTransferParcel {
    pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<AlkaneTransferParcel> {
        let mut result = AlkaneTransferParcel::default();
        for _i in [0..consume_sized_int::<u128>(cursor)?] {
            result.0.push(AlkaneTransfer::parse(cursor)?);
        }
        Ok(result)
    }
    pub fn transfer_from<T: KeyValuePointer>(
        &self,
        pointer: &mut T,
        from: &AlkaneId,
        to: &AlkaneId,
    ) -> Result<()> {
        for transfer in &self.0 {
            let balance = pointer
                .keyword("/alkanes/")
                .select(&transfer.id.into())
                .keyword("/balances/")
                .select(&from.into())
                .get_value::<u128>();
            if balance < transfer.value {
                return Err(anyhow!("balance underflow"));
            }
            pointer
                .keyword("/alkanes/")
                .select(&transfer.id.into())
                .keyword("/balances/")
                .select(&from.into())
                .set_value::<u128>(balance - transfer.value);
            pointer
                .keyword("/alkanes/")
                .select(&transfer.id.into())
                .keyword("/balances/")
                .select(&to.into())
                .set_value::<u128>(balance + transfer.value);
        }
        Ok(())
    }
}

impl AlkaneTransfer {
    pub fn parse(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<AlkaneTransfer> {
        let id = AlkaneId::parse(cursor)?;
        let value = consume_sized_int::<u128>(cursor)?;
        Ok(AlkaneTransfer { id, value })
    }
}
