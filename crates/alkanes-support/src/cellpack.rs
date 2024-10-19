use crate::id::AlkaneId;
use anyhow::Result;
use metashrew_support::utils::consume_sized_int;
use protorune_support::utils::encode_varint_list;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Cellpack {
    pub target: AlkaneId,
    pub inputs: Vec<u128>,
}

impl Cellpack {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<Cellpack> {
        let target = AlkaneId::parse(cursor)?;
        let mut result = Cellpack::default();
        result.target = target;
        loop {
            match consume_sized_int::<u128>(cursor) {
                Ok(v) => result.inputs.push(v),
                Err(_) => {
                    break;
                }
            }
        }
        Ok(result)
    }

    /// Enciphers a Cellpack to a raw protomessage calldata
    pub fn encipher(&self) -> Vec<u8> {
        let mut values = Vec::<u128>::new();
        values.push(self.target.block);
        values.push(self.target.tx);
        values.extend(&self.inputs);
        // leb encode the list
        return encode_varint_list(&values);
    }

    // non LEB encipher if we ever need it
    // pub fn encipher(&self) -> Vec<u8> {
    //     let mut values = Vec::<u8>::new();
    //     values.extend(self.target.block.to_le_bytes());
    //     values.extend(self.target.tx.to_le_bytes());
    //     let inputs_le: Vec<u8> = self
    //         .inputs
    //         .iter()
    //         .flat_map(|&num| num.to_le_bytes()) // Convert each u128 to bytes
    //         .collect();
    //     values.extend(inputs_le);
    //     // leb encode the list
    //     return values;
    // }
}

impl TryFrom<Vec<u128>> for Cellpack {
    type Error = anyhow::Error;
    fn try_from(v: Vec<u128>) -> std::result::Result<Cellpack, Self::Error> {
        Ok(Cellpack {
            target: <[u128; 2] as TryFrom<&[u128]>>::try_from(&v[0..2])?.into(),
            inputs: (&v[2..]).to_vec(),
        })
    }
}
