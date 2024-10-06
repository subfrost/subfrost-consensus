use protorune::message::{MessageContext, MessageContextParcel};

#[derive(Clone, Default)]
pub struct AlkaneMessageContext(());

// TODO: import MessageContextParcel

impl MessageContext for AlkaneMessageContext {
    fn protocol_tag() -> u128 {
        1
    }
    /*
     * TODO: change protorune-rs to supply MessageContextParcel
    fn handle(data: &MessageContextParcel) -> bool {
      true
    }
    */
    fn handle(_parcel: Box<MessageContextParcel>) -> bool {
        true
    }
}

/*
impl MessageContext {
  fn parcel() -> Box<MessageContextParcel> {
    Box::new(MessageContextParcel::default())
  }
}
*/
