// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `neutron/dex/tick_liquidity.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:neutron.dex.TickLiquidity)
pub struct TickLiquidity {
    // message oneof groups
    pub liquidity: ::std::option::Option<tick_liquidity::Liquidity>,
    // special fields
    // @@protoc_insertion_point(special_field:neutron.dex.TickLiquidity.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TickLiquidity {
    fn default() -> &'a TickLiquidity {
        <TickLiquidity as ::protobuf::Message>::default_instance()
    }
}

impl TickLiquidity {
    pub fn new() -> TickLiquidity {
        ::std::default::Default::default()
    }

    // .neutron.dex.PoolReserves pool_reserves = 1;

    pub fn pool_reserves(&self) -> &super::pool_reserves::PoolReserves {
        match self.liquidity {
            ::std::option::Option::Some(tick_liquidity::Liquidity::PoolReserves(ref v)) => v,
            _ => <super::pool_reserves::PoolReserves as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_pool_reserves(&mut self) {
        self.liquidity = ::std::option::Option::None;
    }

    pub fn has_pool_reserves(&self) -> bool {
        match self.liquidity {
            ::std::option::Option::Some(tick_liquidity::Liquidity::PoolReserves(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pool_reserves(&mut self, v: super::pool_reserves::PoolReserves) {
        self.liquidity = ::std::option::Option::Some(tick_liquidity::Liquidity::PoolReserves(v))
    }

    // Mutable pointer to the field.
    pub fn mut_pool_reserves(&mut self) -> &mut super::pool_reserves::PoolReserves {
        if let ::std::option::Option::Some(tick_liquidity::Liquidity::PoolReserves(_)) = self.liquidity {
        } else {
            self.liquidity = ::std::option::Option::Some(tick_liquidity::Liquidity::PoolReserves(super::pool_reserves::PoolReserves::new()));
        }
        match self.liquidity {
            ::std::option::Option::Some(tick_liquidity::Liquidity::PoolReserves(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_pool_reserves(&mut self) -> super::pool_reserves::PoolReserves {
        if self.has_pool_reserves() {
            match self.liquidity.take() {
                ::std::option::Option::Some(tick_liquidity::Liquidity::PoolReserves(v)) => v,
                _ => panic!(),
            }
        } else {
            super::pool_reserves::PoolReserves::new()
        }
    }

    // .neutron.dex.LimitOrderTranche limit_order_tranche = 2;

    pub fn limit_order_tranche(&self) -> &super::limit_order_tranche::LimitOrderTranche {
        match self.liquidity {
            ::std::option::Option::Some(tick_liquidity::Liquidity::LimitOrderTranche(ref v)) => v,
            _ => <super::limit_order_tranche::LimitOrderTranche as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_limit_order_tranche(&mut self) {
        self.liquidity = ::std::option::Option::None;
    }

    pub fn has_limit_order_tranche(&self) -> bool {
        match self.liquidity {
            ::std::option::Option::Some(tick_liquidity::Liquidity::LimitOrderTranche(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_limit_order_tranche(&mut self, v: super::limit_order_tranche::LimitOrderTranche) {
        self.liquidity = ::std::option::Option::Some(tick_liquidity::Liquidity::LimitOrderTranche(v))
    }

    // Mutable pointer to the field.
    pub fn mut_limit_order_tranche(&mut self) -> &mut super::limit_order_tranche::LimitOrderTranche {
        if let ::std::option::Option::Some(tick_liquidity::Liquidity::LimitOrderTranche(_)) = self.liquidity {
        } else {
            self.liquidity = ::std::option::Option::Some(tick_liquidity::Liquidity::LimitOrderTranche(super::limit_order_tranche::LimitOrderTranche::new()));
        }
        match self.liquidity {
            ::std::option::Option::Some(tick_liquidity::Liquidity::LimitOrderTranche(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_limit_order_tranche(&mut self) -> super::limit_order_tranche::LimitOrderTranche {
        if self.has_limit_order_tranche() {
            match self.liquidity.take() {
                ::std::option::Option::Some(tick_liquidity::Liquidity::LimitOrderTranche(v)) => v,
                _ => panic!(),
            }
        } else {
            super::limit_order_tranche::LimitOrderTranche::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::pool_reserves::PoolReserves>(
            "pool_reserves",
            TickLiquidity::has_pool_reserves,
            TickLiquidity::pool_reserves,
            TickLiquidity::mut_pool_reserves,
            TickLiquidity::set_pool_reserves,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::limit_order_tranche::LimitOrderTranche>(
            "limit_order_tranche",
            TickLiquidity::has_limit_order_tranche,
            TickLiquidity::limit_order_tranche,
            TickLiquidity::mut_limit_order_tranche,
            TickLiquidity::set_limit_order_tranche,
        ));
        oneofs.push(tick_liquidity::Liquidity::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TickLiquidity>(
            "TickLiquidity",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TickLiquidity {
    const NAME: &'static str = "TickLiquidity";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.liquidity = ::std::option::Option::Some(tick_liquidity::Liquidity::PoolReserves(is.read_message()?));
                },
                18 => {
                    self.liquidity = ::std::option::Option::Some(tick_liquidity::Liquidity::LimitOrderTranche(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.liquidity {
            match v {
                &tick_liquidity::Liquidity::PoolReserves(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &tick_liquidity::Liquidity::LimitOrderTranche(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.liquidity {
            match v {
                &tick_liquidity::Liquidity::PoolReserves(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &tick_liquidity::Liquidity::LimitOrderTranche(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
            };
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

    fn new() -> TickLiquidity {
        TickLiquidity::new()
    }

    fn clear(&mut self) {
        self.liquidity = ::std::option::Option::None;
        self.liquidity = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TickLiquidity {
        static instance: TickLiquidity = TickLiquidity {
            liquidity: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TickLiquidity {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TickLiquidity").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TickLiquidity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TickLiquidity {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `TickLiquidity`
pub mod tick_liquidity {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:neutron.dex.TickLiquidity.liquidity)
    pub enum Liquidity {
        // @@protoc_insertion_point(oneof_field:neutron.dex.TickLiquidity.pool_reserves)
        PoolReserves(super::super::pool_reserves::PoolReserves),
        // @@protoc_insertion_point(oneof_field:neutron.dex.TickLiquidity.limit_order_tranche)
        LimitOrderTranche(super::super::limit_order_tranche::LimitOrderTranche),
    }

    impl ::protobuf::Oneof for Liquidity {
    }

    impl ::protobuf::OneofFull for Liquidity {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::TickLiquidity as ::protobuf::MessageFull>::descriptor().oneof_by_name("liquidity").unwrap()).clone()
        }
    }

    impl Liquidity {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Liquidity>("liquidity")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20neutron/dex/tick_liquidity.proto\x12\x0bneutron.dex\x1a\x14gogopro\
    to/gogo.proto\x1a%neutron/dex/limit_order_tranche.proto\x1a\x1fneutron/d\
    ex/pool_reserves.proto\"\xb0\x01\n\rTickLiquidity\x12@\n\rpool_reserves\
    \x18\x01\x20\x01(\x0b2\x19.neutron.dex.PoolReservesH\0R\x0cpoolReserves\
    \x12P\n\x13limit_order_tranche\x18\x02\x20\x01(\x0b2\x1e.neutron.dex.Lim\
    itOrderTrancheH\0R\x11limitOrderTrancheB\x0b\n\tliquidityB,Z*github.com/\
    neutron-org/neutron/x/dex/typesJ\xf4\x01\n\x06\x12\x04\0\0\x0f\x01\n\x08\
    \n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\0\x14\n\x08\n\x01\
    \x08\x12\x03\x03\0A\n\t\n\x02\x08\x0b\x12\x03\x03\0A\n\t\n\x02\x03\0\x12\
    \x03\x04\0\x1e\n\t\n\x02\x03\x01\x12\x03\x05\0/\n\t\n\x02\x03\x02\x12\
    \x03\x06\0)\n\n\n\x02\x04\0\x12\x04\t\0\x0f\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\t\x08\x15\n\x0c\n\x04\x04\0\x08\0\x12\x04\n\x02\r\x03\n\x0c\n\x05\
    \x04\0\x08\0\x01\x12\x03\n\x08\x11\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0b\
    \x04#\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x0b\x04\x10\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x0b\x11\x1e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0b!\
    \"\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0c\x04.\n\x0c\n\x05\x04\0\x02\x01\
    \x06\x12\x03\x0c\x04\x15\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0c\x16)\
    \n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0c,-b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::gogo::file_descriptor().clone());
            deps.push(super::limit_order_tranche::file_descriptor().clone());
            deps.push(super::pool_reserves::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TickLiquidity::generated_message_descriptor_data());
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
