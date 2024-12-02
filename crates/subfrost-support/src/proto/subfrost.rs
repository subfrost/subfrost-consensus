// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by protoc 28.2
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `subfrost.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:subfrost.uint128)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Uint128 {
    // message fields
    // @@protoc_insertion_point(field:subfrost.uint128.lo)
    pub lo: u64,
    // @@protoc_insertion_point(field:subfrost.uint128.hi)
    pub hi: u64,
    // special fields
    // @@protoc_insertion_point(special_field:subfrost.uint128.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Uint128 {
    fn default() -> &'a Uint128 {
        <Uint128 as ::protobuf::Message>::default_instance()
    }
}

impl Uint128 {
    pub fn new() -> Uint128 {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "lo",
            |m: &Uint128| { &m.lo },
            |m: &mut Uint128| { &mut m.lo },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hi",
            |m: &Uint128| { &m.hi },
            |m: &mut Uint128| { &mut m.hi },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Uint128>(
            "uint128",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Uint128 {
    const NAME: &'static str = "uint128";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.lo = is.read_uint64()?;
                },
                16 => {
                    self.hi = is.read_uint64()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.lo != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.lo);
        }
        if self.hi != 0 {
            my_size += ::protobuf::rt::uint64_size(2, self.hi);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.lo != 0 {
            os.write_uint64(1, self.lo)?;
        }
        if self.hi != 0 {
            os.write_uint64(2, self.hi)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Uint128 {
        Uint128::new()
    }

    fn clear(&mut self) {
        self.lo = 0;
        self.hi = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Uint128 {
        static instance: Uint128 = Uint128 {
            lo: 0,
            hi: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Uint128 {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("uint128").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Uint128 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Uint128 {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:subfrost.AlkaneId)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AlkaneId {
    // message fields
    // @@protoc_insertion_point(field:subfrost.AlkaneId.block)
    pub block: ::protobuf::MessageField<Uint128>,
    // @@protoc_insertion_point(field:subfrost.AlkaneId.tx)
    pub tx: ::protobuf::MessageField<Uint128>,
    // special fields
    // @@protoc_insertion_point(special_field:subfrost.AlkaneId.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AlkaneId {
    fn default() -> &'a AlkaneId {
        <AlkaneId as ::protobuf::Message>::default_instance()
    }
}

impl AlkaneId {
    pub fn new() -> AlkaneId {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Uint128>(
            "block",
            |m: &AlkaneId| { &m.block },
            |m: &mut AlkaneId| { &mut m.block },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Uint128>(
            "tx",
            |m: &AlkaneId| { &m.tx },
            |m: &mut AlkaneId| { &mut m.tx },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AlkaneId>(
            "AlkaneId",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AlkaneId {
    const NAME: &'static str = "AlkaneId";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.block)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.tx)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.tx.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.block.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.tx.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> AlkaneId {
        AlkaneId::new()
    }

    fn clear(&mut self) {
        self.block.clear();
        self.tx.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AlkaneId {
        static instance: AlkaneId = AlkaneId {
            block: ::protobuf::MessageField::none(),
            tx: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AlkaneId {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AlkaneId").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AlkaneId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlkaneId {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:subfrost.AlkaneTransfer)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AlkaneTransfer {
    // message fields
    // @@protoc_insertion_point(field:subfrost.AlkaneTransfer.id)
    pub id: ::protobuf::MessageField<AlkaneId>,
    // @@protoc_insertion_point(field:subfrost.AlkaneTransfer.value)
    pub value: ::protobuf::MessageField<Uint128>,
    // special fields
    // @@protoc_insertion_point(special_field:subfrost.AlkaneTransfer.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AlkaneTransfer {
    fn default() -> &'a AlkaneTransfer {
        <AlkaneTransfer as ::protobuf::Message>::default_instance()
    }
}

impl AlkaneTransfer {
    pub fn new() -> AlkaneTransfer {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, AlkaneId>(
            "id",
            |m: &AlkaneTransfer| { &m.id },
            |m: &mut AlkaneTransfer| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Uint128>(
            "value",
            |m: &AlkaneTransfer| { &m.value },
            |m: &mut AlkaneTransfer| { &mut m.value },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AlkaneTransfer>(
            "AlkaneTransfer",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AlkaneTransfer {
    const NAME: &'static str = "AlkaneTransfer";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.id)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.value)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.id.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.value.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> AlkaneTransfer {
        AlkaneTransfer::new()
    }

    fn clear(&mut self) {
        self.id.clear();
        self.value.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AlkaneTransfer {
        static instance: AlkaneTransfer = AlkaneTransfer {
            id: ::protobuf::MessageField::none(),
            value: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AlkaneTransfer {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AlkaneTransfer").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AlkaneTransfer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlkaneTransfer {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:subfrost.ReceiptsRequest)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ReceiptsRequest {
    // message fields
    // @@protoc_insertion_point(field:subfrost.ReceiptsRequest.recipient)
    pub recipient: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:subfrost.ReceiptsRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ReceiptsRequest {
    fn default() -> &'a ReceiptsRequest {
        <ReceiptsRequest as ::protobuf::Message>::default_instance()
    }
}

impl ReceiptsRequest {
    pub fn new() -> ReceiptsRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "recipient",
            |m: &ReceiptsRequest| { &m.recipient },
            |m: &mut ReceiptsRequest| { &mut m.recipient },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ReceiptsRequest>(
            "ReceiptsRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ReceiptsRequest {
    const NAME: &'static str = "ReceiptsRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.recipient = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.recipient.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.recipient);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.recipient.is_empty() {
            os.write_bytes(1, &self.recipient)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ReceiptsRequest {
        ReceiptsRequest::new()
    }

    fn clear(&mut self) {
        self.recipient.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ReceiptsRequest {
        static instance: ReceiptsRequest = ReceiptsRequest {
            recipient: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ReceiptsRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ReceiptsRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ReceiptsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReceiptsRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:subfrost.Payment)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Payment {
    // message fields
    // @@protoc_insertion_point(field:subfrost.Payment.sender)
    pub sender: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:subfrost.Payment.transfer)
    pub transfer: ::protobuf::MessageField<AlkaneTransfer>,
    // special fields
    // @@protoc_insertion_point(special_field:subfrost.Payment.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Payment {
    fn default() -> &'a Payment {
        <Payment as ::protobuf::Message>::default_instance()
    }
}

impl Payment {
    pub fn new() -> Payment {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sender",
            |m: &Payment| { &m.sender },
            |m: &mut Payment| { &mut m.sender },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, AlkaneTransfer>(
            "transfer",
            |m: &Payment| { &m.transfer },
            |m: &mut Payment| { &mut m.transfer },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Payment>(
            "Payment",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Payment {
    const NAME: &'static str = "Payment";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.sender = is.read_bytes()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.transfer)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.sender.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.sender);
        }
        if let Some(v) = self.transfer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.sender.is_empty() {
            os.write_bytes(1, &self.sender)?;
        }
        if let Some(v) = self.transfer.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Payment {
        Payment::new()
    }

    fn clear(&mut self) {
        self.sender.clear();
        self.transfer.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Payment {
        static instance: Payment = Payment {
            sender: ::std::vec::Vec::new(),
            transfer: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Payment {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Payment").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Payment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Payment {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:subfrost.ReceiptsResponse)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ReceiptsResponse {
    // message fields
    // @@protoc_insertion_point(field:subfrost.ReceiptsResponse.payments)
    pub payments: ::std::vec::Vec<Payment>,
    // special fields
    // @@protoc_insertion_point(special_field:subfrost.ReceiptsResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ReceiptsResponse {
    fn default() -> &'a ReceiptsResponse {
        <ReceiptsResponse as ::protobuf::Message>::default_instance()
    }
}

impl ReceiptsResponse {
    pub fn new() -> ReceiptsResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "payments",
            |m: &ReceiptsResponse| { &m.payments },
            |m: &mut ReceiptsResponse| { &mut m.payments },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ReceiptsResponse>(
            "ReceiptsResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ReceiptsResponse {
    const NAME: &'static str = "ReceiptsResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.payments.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.payments {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.payments {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ReceiptsResponse {
        ReceiptsResponse::new()
    }

    fn clear(&mut self) {
        self.payments.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ReceiptsResponse {
        static instance: ReceiptsResponse = ReceiptsResponse {
            payments: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ReceiptsResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ReceiptsResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ReceiptsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReceiptsResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0esubfrost.proto\x12\x08subfrost\")\n\x07uint128\x12\x0e\n\x02lo\x18\
    \x01\x20\x01(\x04R\x02lo\x12\x0e\n\x02hi\x18\x02\x20\x01(\x04R\x02hi\"V\
    \n\x08AlkaneId\x12'\n\x05block\x18\x01\x20\x01(\x0b2\x11.subfrost.uint12\
    8R\x05block\x12!\n\x02tx\x18\x02\x20\x01(\x0b2\x11.subfrost.uint128R\x02\
    tx\"]\n\x0eAlkaneTransfer\x12\"\n\x02id\x18\x01\x20\x01(\x0b2\x12.subfro\
    st.AlkaneIdR\x02id\x12'\n\x05value\x18\x02\x20\x01(\x0b2\x11.subfrost.ui\
    nt128R\x05value\"/\n\x0fReceiptsRequest\x12\x1c\n\trecipient\x18\x01\x20\
    \x01(\x0cR\trecipient\"W\n\x07Payment\x12\x16\n\x06sender\x18\x01\x20\
    \x01(\x0cR\x06sender\x124\n\x08transfer\x18\x02\x20\x01(\x0b2\x18.subfro\
    st.AlkaneTransferR\x08transfer\"A\n\x10ReceiptsResponse\x12-\n\x08paymen\
    ts\x18\x01\x20\x03(\x0b2\x11.subfrost.PaymentR\x08paymentsb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(6);
            messages.push(Uint128::generated_message_descriptor_data());
            messages.push(AlkaneId::generated_message_descriptor_data());
            messages.push(AlkaneTransfer::generated_message_descriptor_data());
            messages.push(ReceiptsRequest::generated_message_descriptor_data());
            messages.push(Payment::generated_message_descriptor_data());
            messages.push(ReceiptsResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
