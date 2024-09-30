use crate::utils::read_u128;

#[derive(Default, Clone)]
pub struct AlkaneTransferParcel(pub AlkaneTransfer);

impl AlkaneTransferParcel {
  pub fn parse(cursor: &mut std::io::Cursor) -> AlkaneTransferParcel {
    let mut result = AlkaneTransferParcel::default();
    for i in [0..read_u128(cursor)] {
      result.0.push(AlkaneTransfer::parse(cursor));
    }
    result
  }
}
