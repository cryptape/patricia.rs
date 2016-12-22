// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct RecursiveProto {
    // message fields
    content: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    sub_recursive: ::protobuf::SingularPtrField<RecursiveProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RecursiveProto {}

impl RecursiveProto {
    pub fn new() -> RecursiveProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RecursiveProto {
        static mut instance: ::protobuf::lazy::Lazy<RecursiveProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RecursiveProto,
        };
        unsafe {
            instance.get(|| {
                RecursiveProto {
                    content: ::protobuf::SingularField::none(),
                    sub_recursive: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes content = 1;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.content.is_none() {
            self.content.set_default();
        };
        self.content.as_mut().unwrap()
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::vec::Vec<u8> {
        self.content.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_content(&self) -> &[u8] {
        match self.content.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .RecursiveProto sub_recursive = 2;

    pub fn clear_sub_recursive(&mut self) {
        self.sub_recursive.clear();
    }

    pub fn has_sub_recursive(&self) -> bool {
        self.sub_recursive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sub_recursive(&mut self, v: RecursiveProto) {
        self.sub_recursive = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sub_recursive(&mut self) -> &mut RecursiveProto {
        if self.sub_recursive.is_none() {
            self.sub_recursive.set_default();
        };
        self.sub_recursive.as_mut().unwrap()
    }

    // Take field
    pub fn take_sub_recursive(&mut self) -> RecursiveProto {
        self.sub_recursive.take().unwrap_or_else(|| RecursiveProto::new())
    }

    pub fn get_sub_recursive(&self) -> &RecursiveProto {
        self.sub_recursive.as_ref().unwrap_or_else(|| RecursiveProto::default_instance())
    }
}

impl ::protobuf::Message for RecursiveProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.content));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sub_recursive));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.content {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.sub_recursive {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.content.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.sub_recursive.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RecursiveProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RecursiveProto {
    fn new() -> RecursiveProto {
        RecursiveProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RecursiveProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "content",
                    RecursiveProto::has_content,
                    RecursiveProto::get_content,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "sub_recursive",
                    RecursiveProto::has_sub_recursive,
                    RecursiveProto::get_sub_recursive,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RecursiveProto>(
                    "RecursiveProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RecursiveProto {
    fn clear(&mut self) {
        self.clear_content();
        self.clear_sub_recursive();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RecursiveProto {
    fn eq(&self, other: &RecursiveProto) -> bool {
        self.content == other.content &&
        self.sub_recursive == other.sub_recursive &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RecursiveProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x72, 0x65, 0x63, 0x75, 0x72, 0x73, 0x69, 0x76, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0x60, 0x0a, 0x0e, 0x52, 0x65, 0x63, 0x75, 0x72, 0x73, 0x69, 0x76, 0x65, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x12, 0x34, 0x0a,
    0x0d, 0x73, 0x75, 0x62, 0x5f, 0x72, 0x65, 0x63, 0x75, 0x72, 0x73, 0x69, 0x76, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x52, 0x65, 0x63, 0x75, 0x72, 0x73, 0x69, 0x76, 0x65,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x52, 0x0c, 0x73, 0x75, 0x62, 0x52, 0x65, 0x63, 0x75, 0x72, 0x73,
    0x69, 0x76, 0x65, 0x4a, 0xb6, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x06, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x01, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x03, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x16,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x02, 0x14, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x04, 0x02, 0x03, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x04, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x04, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x05, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x05, 0x02,
    0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x05, 0x02, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x05, 0x11, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x05, 0x21, 0x22, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
