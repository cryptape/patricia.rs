extern crate ethcore_bigint as bigint;
extern crate elastic_array;
extern crate rustc_serialize;
extern crate protobuf;

use protobuf::Message;
use protobuf::error::ProtobufResult;
use protobuf::core::parse_from_bytes;


mod recursive;

#[macro_use]
extern crate lazy_static;

use elastic_array::ElasticArray1024;

/*
pub fn decode<T>(bytes: &[u8]) -> T where T: RlpDecodable {
	let rlp = Rlp::new(bytes);
	rlp.as_val()
}

pub fn encode<E>(object: &E) -> ElasticArray1024<u8> where E: RlpEncodable {
	let mut stream = RlpStream::new();
	stream.append(object);
	stream.drain()
}
*/

use recursive::{ListProto, RecursiveProto};

impl ListProto {
    pub fn append_str(&mut self, str: ::std::vec::Vec<u8>) {
        self.mut_str().push(str);
    }
    pub fn append_list(&mut self, list: ListProto) {
        self.mut_list().push(list);
    }
}
