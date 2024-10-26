use alkanes_runtime::runtime::AlkaneResponder;
use alkanes_support::{parcel::AlkaneTransfer, response::CallResponse, utils::shift};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};

#[derive(Default)]
pub struct MintableAlkane(());

const MINT_AMOUNT: u128 = 50_000;

impl AlkaneResponder for MintableAlkane {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        let mut response = CallResponse::forward(&context.incoming_alkanes);
        if inputs.len() == 1 && shift(&mut inputs).unwrap() == 77 {
            response.alkanes.0.push(AlkaneTransfer {
                id: context.myself.clone(),
                value: MINT_AMOUNT,
            });
        }
        response
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&MintableAlkane::default().execute().serialize());
    to_ptr(&mut response) + 4
}
