extern crate ethcore_bigint as bigint;
extern crate elastic_array;
extern crate rustc_serialize;

use super::super::protobuf::Message as Message;
use super::super::protobuf::error::ProtobufResult as ProtobufResult;
use super::super::protobuf::core::parse_from_bytes as parse_from_bytes;

mod node_pb;
mod recursive;

use elastic_array::ElasticArray1024;
use self::recursive::{ListProto, RecursiveProto};
use self::node_pb::{NodePB};

pub fn pb_decode() -> NodePB{
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

