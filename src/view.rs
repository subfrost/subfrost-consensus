use anyhow::{ Result, anyhow };
use bitcoin::OutPoint;
use metashrew_support::index_pointer::KeyValuePointer;
use ::protorune::{ balance_sheet::load_sheet, tables };
use ::protorune::view::{ outpoint_to_bytes, core_outpoint_to_proto };
use ::protorune::proto::protorune::{ self, OutpointResponse, Output, Outpoint };
use protorune_support::balance_sheet::{ self, BalanceSheet };
use protobuf::{ MessageField, SpecialFields, Message };

pub fn alkane_inventory_response(
    outpoint: &OutPoint,
    protocol_id: u128
) -> Result<OutpointResponse> {
    let outpoint_bytes = outpoint_to_bytes(outpoint)?;
    let balance_sheet: BalanceSheet = load_sheet(
        &tables::RuneTable::for_protocol(protocol_id).OUTPOINT_TO_RUNES.select(&outpoint_bytes)
    );

    let mut height: u128 = tables::RUNES.OUTPOINT_TO_HEIGHT
        .select(&outpoint_bytes)
        .get_value::<u64>()
        .into();
    let mut txindex: u128 = tables::RUNES.HEIGHT_TO_TRANSACTION_IDS
        .select_value::<u64>(height as u64)
        .get_list()
        .into_iter()
        .position(|v| v.as_ref().to_vec() == outpoint.txid.as_byte_array().to_vec())
        .ok_or("")
        .map_err(|_| anyhow!("txid not indexed in table"))? as u128;

    if let Some((rune_id, _)) = balance_sheet.clone().balances.iter().next() {
        height = rune_id.block;
        txindex = rune_id.tx;
    }
    let decoded_output: Output = Output::parse_from_bytes(
        &tables::OUTPOINT_TO_OUTPUT.select(&outpoint_bytes).get().as_ref()
    )?;
    Ok(OutpointResponse {
        balances: MessageField::some(balance_sheet.into()),
        outpoint: MessageField::some(core_outpoint_to_proto(&outpoint)),
        output: MessageField::some(decoded_output),
        height: height as u32,
        txindex: txindex as u32,
        special_fields: SpecialFields::new(),
    })
}
