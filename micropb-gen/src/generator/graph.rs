use std::collections::{BTreeMap, BTreeSet};

use crate::generator::{
    message::{Message, Position},
    r#enum::Enum,
};

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

    fn box_cyclic_dependencies_dfs<'b>(&mut self, start: &'b str, visited: &mut BTreeSet<&'b str>)
    where
        'a: 'b,
    {
        let mut edges = vec![DfsElem::Edge(start)];
        let mut ancestors = BTreeSet::new();

        while let Some(elem) = edges.pop() {
            match elem {
                DfsElem::Edge(cur_field) => {
                    if visited.contains(cur_field) {
                        continue;
                    }
                    ancestors.insert(cur_field);
                    edges.push(DfsElem::NodeEnd(cur_field));
                    visited.insert(cur_field);

                    let cur_msg = self
                        .messages
                        .get_mut(cur_field)
                        .unwrap_or_else(|| panic!("{cur_field} not found"));
                    for i in 0..cur_msg.message_edges.len() {
                        let (pos, next_field) = cur_msg.message_edges[i];
                        let is_bool = pos.is_boxed_mut(cur_msg);
                        if !*is_bool {
                            if ancestors.contains(next_field) {
                                *is_bool = true;
                            } else {
                                edges.push(DfsElem::Edge(next_field));
                            }
                        }
                    }
                }
                DfsElem::NodeEnd(field) => {
                    ancestors.remove(field);
                }
            }
        }
    }

    fn box_cyclic_dependencies(&mut self) {
        let mut visited = BTreeSet::new();
        let fields: Vec<_> = self.messages.keys().cloned().collect();

        for field in &fields {
            self.box_cyclic_dependencies_dfs(field, &mut visited);
        }
    }
}

enum DfsElem<'a> {
    Edge(&'a str),
    NodeEnd(&'a str),
}

#[cfg(test)]
mod tests {
    use crate::{
        config::OptionalRepr,
        generator::{
            field::{make_test_field, FieldType},
            message::make_test_msg,
            type_spec::TypeSpec,
        },
    };

    use super::*;

    fn add_msg_field<'a>(
        msg: &mut Message<'a>,
        num: u32,
        fname: &'a str,
        type_name: &'a str,
        boxed: bool,
    ) {
        msg.message_edges
            .push((Position::Field(msg.fields.len()), type_name));
        msg.fields.push(make_test_field(
            num,
            fname,
            boxed,
            FieldType::Optional(TypeSpec::Message(type_name, None), OptionalRepr::Option),
        ));
    }

    #[test]
    fn box_cyclic_dependencies() {
        let mut alpha = make_test_msg("Alpha");
        add_msg_field(&mut alpha, 1, "beta", ".pkg.Beta", false);

        let mut beta = make_test_msg("Beta");
        add_msg_field(&mut beta, 1, "gamma", ".pkg.Gamma", false);
        add_msg_field(&mut beta, 2, "omega", ".pkg.Omega", true);

        let mut gamma = make_test_msg("Gamma");
        add_msg_field(&mut gamma, 1, "alpha", ".pkg.Alpha", false);

        let mut omega = make_test_msg("Omega");
        add_msg_field(&mut omega, 1, "alpha", ".pkg.Alpha", false);

        // self-referential
        let mut sigma = make_test_msg("Sigma");
        add_msg_field(&mut sigma, 1, "sigma", ".pkg.Sigma", false);

        let mut graph = TypeGraph::default();
        graph.add_message(".pkg.Alpha".to_owned(), alpha);
        graph.add_message(".pkg.Beta".to_owned(), beta);
        graph.add_message(".pkg.Gamma".to_owned(), gamma);
        graph.add_message(".pkg.Omega".to_owned(), omega);
        graph.add_message(".pkg.Sigma".to_owned(), sigma);

        graph.box_cyclic_dependencies();

        let alpha = graph.get_message(".pkg.Alpha").unwrap();
        assert!(!alpha.fields[0].boxed);
        let beta = graph.get_message(".pkg.Beta").unwrap();
        assert!(!beta.fields[0].boxed);
        assert!(beta.fields[1].boxed);
        let gamma = graph.get_message(".pkg.Gamma").unwrap();
        assert!(gamma.fields[0].boxed); // Gamma.alpha should have been boxed
        let omega = graph.get_message(".pkg.Omega").unwrap();
        assert!(!omega.fields[0].boxed); // Omega.alpha should stay unboxed, since Beta.omega was already boxed
        let sigma = graph.get_message(".pkg.Sigma").unwrap();
        assert!(sigma.fields[0].boxed); // Sigma.sigma should have been boxed
    }
}
