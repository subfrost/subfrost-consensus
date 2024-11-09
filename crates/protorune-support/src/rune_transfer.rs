use std::collections::HashMap;

use crate::balance_sheet::{BalanceSheet, ProtoruneRuneId};
use anyhow::{anyhow, Result};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RuneTransfer {
    pub id: ProtoruneRuneId,
    pub value: u128,
}

impl RuneTransfer {
    pub fn from_balance_sheet(s: BalanceSheet) -> Vec<Self> {
        s.balances
            .iter()
            .map(|(id, v)| Self {
                id: id.clone(),
                value: *v,
            })
            .collect::<Vec<RuneTransfer>>()
    }
}

pub trait OutgoingRunes {
    fn reconcile(
        &self,
        balances_by_output: &mut HashMap<u32, BalanceSheet>,
        vout: u32,
        pointer: u32,
        refund_pointer: u32,
    ) -> Result<()>;
}

impl OutgoingRunes for (Vec<RuneTransfer>, BalanceSheet) {
    fn reconcile(
        &self,
        balances_by_output: &mut HashMap<u32, BalanceSheet>,
        vout: u32,
        pointer: u32,
        refund_pointer: u32,
    ) -> Result<()> {
        let runtime_initial = balances_by_output
            .get(&u32::MAX)
            .map(|v| v.clone())
            .unwrap_or_else(|| BalanceSheet::default());
        let incoming_initial = balances_by_output
            .get(&vout)
            .ok_or("")
            .map_err(|_| anyhow!("balance sheet not found"))?
            .clone();
        let mut initial = BalanceSheet::merge(&incoming_initial, &runtime_initial);

        // self.0 is the amount to forward to the pointer
        // self.1 is the amount to put into the runtime balance
        let outgoing: BalanceSheet = self.0.clone().into();
        let outgoing_runtime = self.1.clone();

        // we want to subtract outgoing and the outgoing runtime balance
        // amount from the initial amount
        initial.mintable_debit(&outgoing)?;
        initial.mintable_debit(&outgoing_runtime)?;

        // increase the pointer by the outgoing runes balancesheet
        increase_balances_using_sheet(balances_by_output, &outgoing, pointer);

        // set the runtime to the ending runtime balance sheet
        // note that u32::MAX is the runtime vout
        balances_by_output.insert(u32::MAX, outgoing_runtime);

        // refund the remaining amount to the refund pointer
        increase_balances_using_sheet(balances_by_output, &initial, refund_pointer);
        Ok(())
    }
}

/// Parameters:
///   balances_by_output: The running store of balances by each transaction output for
///                       the current transaction being handled.
///   sheet: The balance sheet to increase the balances by
///   vout: The target transaction vout to receive the runes
fn increase_balances_using_sheet(
    balances_by_output: &mut HashMap<u32, BalanceSheet>,
    sheet: &BalanceSheet,
    vout: u32,
) {
    if !balances_by_output.contains_key(&vout) {
        balances_by_output.insert(vout, BalanceSheet::default());
    }
    sheet.pipe(balances_by_output.get_mut(&vout).unwrap());
}

/// Refunds all input runes to the refund pointer
pub fn refund_to_refund_pointer(
    balances_by_output: &mut HashMap<u32, BalanceSheet>,
    protomessage_vout: u32,
    refund_pointer: u32,
) {
    // grab the balance of the protomessage vout
    let sheet = balances_by_output
        .get(&protomessage_vout)
        .map(|v| v.clone())
        .unwrap_or_else(|| BalanceSheet::default());
    // we want to remove any balance from the protomessage vout
    balances_by_output.remove(&protomessage_vout);
    increase_balances_using_sheet(balances_by_output, &sheet, refund_pointer);
}
