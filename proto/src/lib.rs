extern crate ethcore_bigint as bigint;
extern crate elastic_array;
extern crate rustc_serialize;
extern crate protobuf;

use protobuf::Message;
use protobuf::error::ProtobufResult;
use protobuf::core::parse_from_bytes;

mod node_pb;
mod recursive;

#[macro_use]
extern crate lazy_static;
use elastic_array::ElasticArray1024;
use recursive::{ListProto, RecursiveProto};
use node_pb::{NodePB};

pub fn pb_decode(bytes: &[u8]) -> NodePB{
}

pub fn pb_encode() -> NodePB {
}

impl ListProto {
    pub fn append_str(&mut self, str: ::std::vec::Vec<u8>) {
        self.mut_str().push(str);
    }
    pub fn append_list(&mut self, list: ListProto) {
        self.mut_list().push(list);
    }
}

