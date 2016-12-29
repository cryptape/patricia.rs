extern crate ethcore_bigint as bigint;
extern crate elastic_array;
extern crate rustc_serialize;

use super::super::protobuf::Message;
use super::super::protobuf::error::ProtobufResult;
use super::super::protobuf::core::parse_from_bytes;

mod node_pb;
mod recursive;

use elastic_array::ElasticArray1024;
use self::recursive::{ListProto, RecursiveProto};
use self::node_pb::*;
use super::node::Node;
use super::node::NodeKey;
use super::super::hashdb::DBValue;

impl Node {
    pub fn pb_decode(bytes: &[u8]) -> ProtobufResult<Option<Self>> {
        parse_from_bytes::<NodePB>(bytes).map(|proto| proto.into_node())
    }

    pub fn pb_encode(node: &Node) -> ProtobufResult<Vec<u8>> {
        NodePB::from_node(node).write_to_bytes()
    }
}

impl NodePB {
    pub fn into_node(mut self) -> Option<Node> {
        match self.content.unwrap() {
            NodePB_oneof_content::Empty(b) => { Some(Node::Empty) }, 
            NodePB_oneof_content::Leaf(leaf) => { Some(Node::Leaf(NodeKey::from_slice(leaf.get_key()), DBValue::from_slice(leaf.get_value()))) },
            NodePB_oneof_content::Extension(extension) => { Some(Node::Extension(NodeKey::from_slice(extension.get_key()), DBValue::from_slice(extension.get_value()))) },
            NodePB_oneof_content::Branch(branch) => {
                let mut nodes = [NodeKey::new(), NodeKey::new(), NodeKey::new(), NodeKey::new(), NodeKey::new(),
                    NodeKey::new(), NodeKey::new(), NodeKey::new(), NodeKey::new(), NodeKey::new(), NodeKey::new(),
                    NodeKey::new(), NodeKey::new(), NodeKey::new(), NodeKey::new(), NodeKey::new()];
                for i in 0 .. 16 { nodes[i] = NodeKey::from_slice(&branch.get_key()[i]); }
                Some(Node::Branch(nodes, Some(DBValue::from_slice(branch.get_value()))))
            }
        }
    } 

    pub fn from_node(node: &Node) -> Self {
        let mut proto = Self::new();
        match node.clone() {
            Node::Empty => proto.set_Empty(true),
            Node::Leaf(k, v) => {let mut leaf = LeafPB::new(); leaf.set_key(k.to_vec()); leaf.set_value(v.to_vec()); proto.set_Leaf(leaf);},
            Node::Extension(k, v) => {let mut extension = ExtensionPB::new(); extension.set_key(k.to_vec()); extension.set_value(v.to_vec()); proto.set_Extension(extension);},
            Node::Branch(k, v) => {
                let mut branch = BranchPB::new();
                for i in 0 .. 16 {
                    branch.mut_key().push(k[i].clone().to_vec());
                }
                branch.set_value(v.unwrap().to_vec());
                proto.set_Branch(branch);
            }
        }
        proto
    }
}

impl ListProto {
    pub fn append_str(&mut self, str: ::std::vec::Vec<u8>) {
        self.mut_str().push(str);
    }
    pub fn append_list(&mut self, list: ListProto) {
        self.mut_list().push(list);
    }
}

