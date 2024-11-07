use metashrew::index_pointer::IndexPointer;
use metashrew_support::index_pointer::KeyValuePointer;
use once_cell::sync::Lazy;

pub use protorune_support::tables::RuneTable;
pub static RUNES: Lazy<RuneTable> = Lazy::new(|| RuneTable::new());

pub static HEIGHT_TO_RUNES: Lazy<IndexPointer> =
    Lazy::new(|| IndexPointer::from_keyword("/runes/byheight/"));

pub static OUTPOINTS_FOR_ADDRESS: Lazy<IndexPointer> =
    Lazy::new(|| IndexPointer::from_keyword("/outpoint/byaddress/"));

pub static OUTPOINT_SPENDABLE_BY: Lazy<IndexPointer> =
    Lazy::new(|| IndexPointer::from_keyword("/outpoint/spendableby/"));
pub static OUTPOINT_TO_OUTPUT: Lazy<IndexPointer> =
    Lazy::new(|| IndexPointer::from_keyword("/output/byoutpoint/"));
