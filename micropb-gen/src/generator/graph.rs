use std::collections::BTreeMap;

use crate::generator::{message::Message, r#enum::Enum};

#[derive(Default)]
pub(super) struct TypeGraph<'a> {
    messages: BTreeMap<String, Message<'a>>,
    enums: BTreeMap<String, Enum<'a>>,
}

impl<'a> TypeGraph<'a> {
    pub(super) fn add_message(&mut self, fq_proto_name: String, msg: Message<'a>) {
        self.messages.insert(fq_proto_name, msg);
    }

    pub(super) fn add_enum(&mut self, fq_proto_name: String, e: Enum<'a>) {
        self.enums.insert(fq_proto_name, e);
    }

    pub(super) fn get_message(&self, fq_proto_name: &str) -> Option<&Message<'a>> {
        self.messages.get(fq_proto_name)
    }

    pub(super) fn get_enum(&self, fq_proto_name: &str) -> Option<&Enum<'a>> {
        self.enums.get(fq_proto_name)
    }
}
