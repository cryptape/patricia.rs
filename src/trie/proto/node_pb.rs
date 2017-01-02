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
pub struct NodeHandlePB {
    // message oneof groups
    content: ::std::option::Option<NodeHandlePB_oneof_content>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodeHandlePB {}

#[derive(Clone,PartialEq)]
pub enum NodeHandlePB_oneof_content {
    hash(::std::vec::Vec<u8>),
    node(NodePB),
}

impl NodeHandlePB {
    pub fn new() -> NodeHandlePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeHandlePB {
        static mut instance: ::protobuf::lazy::Lazy<NodeHandlePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeHandlePB,
        };
        unsafe {
            instance.get(|| {
                NodeHandlePB {
                    content: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes hash = 1;

    pub fn clear_hash(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(NodeHandlePB_oneof_content::hash(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = ::std::option::Option::Some(NodeHandlePB_oneof_content::hash(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(NodeHandlePB_oneof_content::hash(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(NodeHandlePB_oneof_content::hash(::std::vec::Vec::new()));
        }
        match self.content {
            ::std::option::Option::Some(NodeHandlePB_oneof_content::hash(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_hash() {
            match self.content.take() {
                ::std::option::Option::Some(NodeHandlePB_oneof_content::hash(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_hash(&self) -> &[u8] {
        match self.content {
            ::std::option::Option::Some(NodeHandlePB_oneof_content::hash(ref v)) => v,
            _ => &[],
        }
    }

    // optional .NodePB node = 2;

    pub fn clear_node(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_node(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(NodeHandlePB_oneof_content::node(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: NodePB) {
        self.content = ::std::option::Option::Some(NodeHandlePB_oneof_content::node(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node(&mut self) -> &mut NodePB {
        if let ::std::option::Option::Some(NodeHandlePB_oneof_content::node(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(NodeHandlePB_oneof_content::node(NodePB::new()));
        }
        match self.content {
            ::std::option::Option::Some(NodeHandlePB_oneof_content::node(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_node(&mut self) -> NodePB {
        if self.has_node() {
            match self.content.take() {
                ::std::option::Option::Some(NodeHandlePB_oneof_content::node(v)) => v,
                _ => panic!(),
            }
        } else {
            NodePB::new()
        }
    }

    pub fn get_node(&self) -> &NodePB {
        match self.content {
            ::std::option::Option::Some(NodeHandlePB_oneof_content::node(ref v)) => v,
            _ => NodePB::default_instance(),
        }
    }
}

impl ::protobuf::Message for NodeHandlePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(NodeHandlePB_oneof_content::hash(try!(is.read_bytes())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(NodeHandlePB_oneof_content::node(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &NodeHandlePB_oneof_content::hash(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(1, &v);
                },
                &NodeHandlePB_oneof_content::node(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &NodeHandlePB_oneof_content::hash(ref v) => {
                    try!(os.write_bytes(1, v));
                },
                &NodeHandlePB_oneof_content::node(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
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
        ::std::any::TypeId::of::<NodeHandlePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NodeHandlePB {
    fn new() -> NodeHandlePB {
        NodeHandlePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeHandlePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "hash",
                    NodeHandlePB::has_hash,
                    NodeHandlePB::get_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "node",
                    NodeHandlePB::has_node,
                    NodeHandlePB::get_node,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeHandlePB>(
                    "NodeHandlePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeHandlePB {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_node();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NodeHandlePB {
    fn eq(&self, other: &NodeHandlePB) -> bool {
        self.content == other.content &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NodeHandlePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LeafPB {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LeafPB {}

impl LeafPB {
    pub fn new() -> LeafPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LeafPB {
        static mut instance: ::protobuf::lazy::Lazy<LeafPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LeafPB,
        };
        unsafe {
            instance.get(|| {
                LeafPB {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for LeafPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<LeafPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LeafPB {
    fn new() -> LeafPB {
        LeafPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<LeafPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    LeafPB::has_key,
                    LeafPB::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    LeafPB::has_value,
                    LeafPB::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LeafPB>(
                    "LeafPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LeafPB {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LeafPB {
    fn eq(&self, other: &LeafPB) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LeafPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExtensionPB {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularPtrField<NodeHandlePB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExtensionPB {}

impl ExtensionPB {
    pub fn new() -> ExtensionPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExtensionPB {
        static mut instance: ::protobuf::lazy::Lazy<ExtensionPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExtensionPB,
        };
        unsafe {
            instance.get(|| {
                ExtensionPB {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .NodeHandlePB value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: NodeHandlePB) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut NodeHandlePB {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> NodeHandlePB {
        self.value.take().unwrap_or_else(|| NodeHandlePB::new())
    }

    pub fn get_value(&self) -> &NodeHandlePB {
        self.value.as_ref().unwrap_or_else(|| NodeHandlePB::default_instance())
    }
}

impl ::protobuf::Message for ExtensionPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value));
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.value {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
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
        ::std::any::TypeId::of::<ExtensionPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExtensionPB {
    fn new() -> ExtensionPB {
        ExtensionPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExtensionPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    ExtensionPB::has_key,
                    ExtensionPB::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    ExtensionPB::has_value,
                    ExtensionPB::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExtensionPB>(
                    "ExtensionPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExtensionPB {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExtensionPB {
    fn eq(&self, other: &ExtensionPB) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExtensionPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BranchPB {
    // message fields
    key: ::protobuf::RepeatedField<NodeHandlePB>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BranchPB {}

impl BranchPB {
    pub fn new() -> BranchPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BranchPB {
        static mut instance: ::protobuf::lazy::Lazy<BranchPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BranchPB,
        };
        unsafe {
            instance.get(|| {
                BranchPB {
                    key: ::protobuf::RepeatedField::new(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .NodeHandlePB key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::protobuf::RepeatedField<NodeHandlePB>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    pub fn mut_key(&mut self) -> &mut ::protobuf::RepeatedField<NodeHandlePB> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::protobuf::RepeatedField<NodeHandlePB> {
        ::std::mem::replace(&mut self.key, ::protobuf::RepeatedField::new())
    }

    pub fn get_key(&self) -> &[NodeHandlePB] {
        &self.key
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for BranchPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
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
        for value in &self.key {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.value {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.key {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<BranchPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BranchPB {
    fn new() -> BranchPB {
        BranchPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<BranchPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "key",
                    BranchPB::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    BranchPB::has_value,
                    BranchPB::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BranchPB>(
                    "BranchPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BranchPB {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BranchPB {
    fn eq(&self, other: &BranchPB) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BranchPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct NodePB {
    // message oneof groups
    content: ::std::option::Option<NodePB_oneof_content>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodePB {}

#[derive(Clone,PartialEq)]
pub enum NodePB_oneof_content {
    Empty(bool),
    Leaf(LeafPB),
    Extension(ExtensionPB),
    Branch(BranchPB),
}

impl NodePB {
    pub fn new() -> NodePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodePB {
        static mut instance: ::protobuf::lazy::Lazy<NodePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodePB,
        };
        unsafe {
            instance.get(|| {
                NodePB {
                    content: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool Empty = 1;

    pub fn clear_Empty(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_Empty(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(NodePB_oneof_content::Empty(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_Empty(&mut self, v: bool) {
        self.content = ::std::option::Option::Some(NodePB_oneof_content::Empty(v))
    }

    pub fn get_Empty(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(NodePB_oneof_content::Empty(v)) => v,
            _ => false,
        }
    }

    // optional .LeafPB Leaf = 2;

    pub fn clear_Leaf(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_Leaf(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(NodePB_oneof_content::Leaf(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_Leaf(&mut self, v: LeafPB) {
        self.content = ::std::option::Option::Some(NodePB_oneof_content::Leaf(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Leaf(&mut self) -> &mut LeafPB {
        if let ::std::option::Option::Some(NodePB_oneof_content::Leaf(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(NodePB_oneof_content::Leaf(LeafPB::new()));
        }
        match self.content {
            ::std::option::Option::Some(NodePB_oneof_content::Leaf(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_Leaf(&mut self) -> LeafPB {
        if self.has_Leaf() {
            match self.content.take() {
                ::std::option::Option::Some(NodePB_oneof_content::Leaf(v)) => v,
                _ => panic!(),
            }
        } else {
            LeafPB::new()
        }
    }

    pub fn get_Leaf(&self) -> &LeafPB {
        match self.content {
            ::std::option::Option::Some(NodePB_oneof_content::Leaf(ref v)) => v,
            _ => LeafPB::default_instance(),
        }
    }

    // optional .ExtensionPB Extension = 3;

    pub fn clear_Extension(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_Extension(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(NodePB_oneof_content::Extension(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_Extension(&mut self, v: ExtensionPB) {
        self.content = ::std::option::Option::Some(NodePB_oneof_content::Extension(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Extension(&mut self) -> &mut ExtensionPB {
        if let ::std::option::Option::Some(NodePB_oneof_content::Extension(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(NodePB_oneof_content::Extension(ExtensionPB::new()));
        }
        match self.content {
            ::std::option::Option::Some(NodePB_oneof_content::Extension(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_Extension(&mut self) -> ExtensionPB {
        if self.has_Extension() {
            match self.content.take() {
                ::std::option::Option::Some(NodePB_oneof_content::Extension(v)) => v,
                _ => panic!(),
            }
        } else {
            ExtensionPB::new()
        }
    }

    pub fn get_Extension(&self) -> &ExtensionPB {
        match self.content {
            ::std::option::Option::Some(NodePB_oneof_content::Extension(ref v)) => v,
            _ => ExtensionPB::default_instance(),
        }
    }

    // optional .BranchPB Branch = 4;

    pub fn clear_Branch(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_Branch(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(NodePB_oneof_content::Branch(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_Branch(&mut self, v: BranchPB) {
        self.content = ::std::option::Option::Some(NodePB_oneof_content::Branch(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Branch(&mut self) -> &mut BranchPB {
        if let ::std::option::Option::Some(NodePB_oneof_content::Branch(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(NodePB_oneof_content::Branch(BranchPB::new()));
        }
        match self.content {
            ::std::option::Option::Some(NodePB_oneof_content::Branch(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_Branch(&mut self) -> BranchPB {
        if self.has_Branch() {
            match self.content.take() {
                ::std::option::Option::Some(NodePB_oneof_content::Branch(v)) => v,
                _ => panic!(),
            }
        } else {
            BranchPB::new()
        }
    }

    pub fn get_Branch(&self) -> &BranchPB {
        match self.content {
            ::std::option::Option::Some(NodePB_oneof_content::Branch(ref v)) => v,
            _ => BranchPB::default_instance(),
        }
    }
}

impl ::protobuf::Message for NodePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(NodePB_oneof_content::Empty(try!(is.read_bool())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(NodePB_oneof_content::Leaf(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(NodePB_oneof_content::Extension(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.content = ::std::option::Option::Some(NodePB_oneof_content::Branch(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &NodePB_oneof_content::Empty(v) => {
                    my_size += 2;
                },
                &NodePB_oneof_content::Leaf(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &NodePB_oneof_content::Extension(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &NodePB_oneof_content::Branch(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &NodePB_oneof_content::Empty(v) => {
                    try!(os.write_bool(1, v));
                },
                &NodePB_oneof_content::Leaf(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &NodePB_oneof_content::Extension(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &NodePB_oneof_content::Branch(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
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
        ::std::any::TypeId::of::<NodePB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NodePB {
    fn new() -> NodePB {
        NodePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "Empty",
                    NodePB::has_Empty,
                    NodePB::get_Empty,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "Leaf",
                    NodePB::has_Leaf,
                    NodePB::get_Leaf,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "Extension",
                    NodePB::has_Extension,
                    NodePB::get_Extension,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "Branch",
                    NodePB::has_Branch,
                    NodePB::get_Branch,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodePB>(
                    "NodePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodePB {
    fn clear(&mut self) {
        self.clear_Empty();
        self.clear_Leaf();
        self.clear_Extension();
        self.clear_Branch();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NodePB {
    fn eq(&self, other: &NodePB) -> bool {
        self.content == other.content &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NodePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NodePB_name {
    EMPTY = 0,
    LEAF = 1,
    EXTENSION = 2,
    BRANCH = 3,
}

impl ::protobuf::ProtobufEnum for NodePB_name {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NodePB_name> {
        match value {
            0 => ::std::option::Option::Some(NodePB_name::EMPTY),
            1 => ::std::option::Option::Some(NodePB_name::LEAF),
            2 => ::std::option::Option::Some(NodePB_name::EXTENSION),
            3 => ::std::option::Option::Some(NodePB_name::BRANCH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NodePB_name] = &[
            NodePB_name::EMPTY,
            NodePB_name::LEAF,
            NodePB_name::EXTENSION,
            NodePB_name::BRANCH,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<NodePB_name>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NodePB_name", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NodePB_name {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x4e, 0x0a, 0x0c, 0x4e, 0x6f, 0x64, 0x65, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x50, 0x42, 0x12,
    0x14, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00, 0x52,
    0x04, 0x68, 0x61, 0x73, 0x68, 0x12, 0x1d, 0x0a, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x50, 0x42, 0x48, 0x00, 0x52, 0x04,
    0x6e, 0x6f, 0x64, 0x65, 0x42, 0x09, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x22,
    0x30, 0x0a, 0x06, 0x4c, 0x65, 0x61, 0x66, 0x50, 0x42, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x22, 0x44, 0x0a, 0x0b, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x42,
    0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b,
    0x65, 0x79, 0x12, 0x23, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0d, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x50, 0x42,
    0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x41, 0x0a, 0x08, 0x42, 0x72, 0x61, 0x6e, 0x63,
    0x68, 0x50, 0x42, 0x12, 0x1f, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x0d, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x50, 0x42, 0x52,
    0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xd5, 0x01, 0x0a, 0x06, 0x4e,
    0x6f, 0x64, 0x65, 0x50, 0x42, 0x12, 0x16, 0x0a, 0x05, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x05, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x12, 0x1d, 0x0a,
    0x04, 0x4c, 0x65, 0x61, 0x66, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x4c, 0x65,
    0x61, 0x66, 0x50, 0x42, 0x48, 0x00, 0x52, 0x04, 0x4c, 0x65, 0x61, 0x66, 0x12, 0x2c, 0x0a, 0x09,
    0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0c, 0x2e, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x42, 0x48, 0x00, 0x52,
    0x09, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x23, 0x0a, 0x06, 0x42, 0x72,
    0x61, 0x6e, 0x63, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x42, 0x72, 0x61,
    0x6e, 0x63, 0x68, 0x50, 0x42, 0x48, 0x00, 0x52, 0x06, 0x42, 0x72, 0x61, 0x6e, 0x63, 0x68, 0x22,
    0x36, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x09, 0x0a, 0x05, 0x45, 0x4d, 0x50, 0x54, 0x59,
    0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x4c, 0x45, 0x41, 0x46, 0x10, 0x01, 0x12, 0x0d, 0x0a, 0x09,
    0x45, 0x58, 0x54, 0x45, 0x4e, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x02, 0x12, 0x0a, 0x0a, 0x06, 0x42,
    0x52, 0x41, 0x4e, 0x43, 0x48, 0x10, 0x03, 0x42, 0x09, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x4a, 0x87, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x26, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x01, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03,
    0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x14, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x04, 0x04, 0x07, 0x05, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x04, 0x0a, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x05, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x05, 0x0e, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05,
    0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x08, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x06, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x0a, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08,
    0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x04, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0b, 0x04, 0x0a, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x0a, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0b, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x0c, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0c,
    0x04, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x04,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x0a, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x12, 0x13, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x0f, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x0f, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x10,
    0x04, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x10, 0x04, 0x0f,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x10, 0x04, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x0a, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x11, 0x04, 0x10, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x11, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x11, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x19,
    0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x14, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x14, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x00, 0x12, 0x03, 0x15, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x15, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x15,
    0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x1a, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x20, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x16, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x16, 0x04, 0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x16, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x16, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x16, 0x12, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x19, 0x00, 0x26, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x19, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x04, 0x04, 0x00, 0x12, 0x04, 0x1a, 0x04, 0x1f, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x09, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x1b, 0x08, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x1b, 0x10, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x1c, 0x08, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x1c, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x1c, 0x0f, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x1d, 0x08, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x1d, 0x08, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x1d, 0x14, 0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x1e, 0x08, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x1e, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x1e, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x08, 0x00, 0x12, 0x04, 0x20, 0x04,
    0x25, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x08, 0x00, 0x01, 0x12, 0x03, 0x20, 0x0a, 0x11,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x21, 0x08, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x21, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x21, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x22, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x22,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22, 0x0f, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x16, 0x17, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x23, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x23, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x23, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x23, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x24,
    0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x06, 0x12, 0x03, 0x24, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x24, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x24, 0x1a, 0x1b, 0x62, 0x06, 0x70, 0x72,
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
