use std::collections::{BTreeMap, BTreeSet};

use crate::generator::{r#enum::Enum, field::FieldType, message::Message};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[derive(Clone, Copy)]
pub(crate) enum Position {
    Field(usize),
    Oneof(usize, usize),
}

impl Position {
    pub(crate) fn is_boxed_mut<'a>(&self, msg: &'a mut Message) -> Option<&'a mut bool> {
        match self {
            Position::Field(i) => {
                let field = &mut msg.fields[*i];
                // For the purpose of cycle detection, ignore boxing repeated and map fields since
                // those fields are typically allocated on the heap. The exception is no-std, where
                // these fields are statically-allocated, but boxing isn't relevant there anyways.
                if let FieldType::Map { .. } | FieldType::Repeated { .. } = field.ftype {
                    None
                } else {
                    Some(&mut field.boxed)
                }
            }
            Position::Oneof(oi, fi) => Some(
                &mut msg.oneofs[*oi]
                    .otype
                    .fields_mut()
                    .expect("unexpected custom oneof")[*fi]
                    .boxed,
            ),
        }
    }

    pub(crate) fn max_size_override_mut<'a>(
        &self,
        msg: &'a mut Message,
    ) -> Option<&'a mut Option<Option<usize>>> {
        match self {
            Position::Field(i) => {
                let field = &mut msg.fields[*i];
                // If the field is an unbounded container, then its MAX_SIZE is going to be
                // None, so the field can be ignored for cycle detection
                if let FieldType::Map { max_len: None, .. }
                | FieldType::Repeated { max_len: None, .. } = field.ftype
                {
                    None
                } else {
                    Some(&mut field.max_size_override)
                }
            }
            Position::Oneof(oi, fi) => Some(
                &mut msg.oneofs[*oi]
                    .otype
                    .fields_mut()
                    .expect("unexpected custom oneof")[*fi]
                    .max_size_override,
            ),
        }
    }
}

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

    fn cycle_breaker_dfs<'b, T>(
        &mut self,
        start: &'b str,
        visited: &mut BTreeSet<&'b str>,
        get_property: impl for<'p> Fn(&Position, &'p mut Message) -> Option<&'p mut T>,
        ignore_edge: impl Fn(&T) -> bool,
        break_cycle: impl Fn(&mut T),
    ) where
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
                        let prop = get_property(&pos, cur_msg);
                        if let Some(prop) = prop
                            && !ignore_edge(prop)
                        {
                            if ancestors.contains(next_field) {
                                break_cycle(prop);
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

    /// Detect cycles in the message graph via DFS and break those cycles by boxing fields.
    pub(crate) fn box_cyclic_dependencies(&mut self) {
        let mut visited = BTreeSet::new();
        let fields: Vec<_> = self.messages.keys().cloned().collect();

        for field in &fields {
            self.cycle_breaker_dfs(
                field,
                &mut visited,
                |pos, msg| pos.is_boxed_mut(msg),
                |boxed| *boxed,
                |boxed| *boxed = true,
            );
        }
    }

    /// Detect cycles in the message graph via DFS and break those cycles by overriding max size
    pub(crate) fn max_size_cyclic_dependencies(&mut self) {
        let mut visited = BTreeSet::new();
        let fields: Vec<_> = self.messages.keys().cloned().collect();

        for field in &fields {
            self.cycle_breaker_dfs(
                field,
                &mut visited,
                |pos, msg| pos.max_size_override_mut(msg),
                |max_size_override| max_size_override.is_some(),
                // Break the cycle by setting MAX_SIZE to None, resulting in the MAX_SIZE of all
                // messages in the cycle to become None
                |max_size_override| *max_size_override = Some(None),
            );
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
            field::{FieldType, make_test_field},
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
        max_size_override: Option<Option<usize>>,
    ) {
        msg.message_edges
            .push((Position::Field(msg.fields.len()), type_name));
        let mut field = make_test_field(
            num,
            fname,
            boxed,
            FieldType::Optional(TypeSpec::Message(type_name, None), OptionalRepr::Option),
        );
        field.max_size_override = max_size_override;
        msg.fields.push(field);
    }

    #[test]
    fn box_cyclic_dependencies() {
        let mut alpha = make_test_msg("Alpha");
        add_msg_field(&mut alpha, 1, "beta", ".pkg.Beta", false, None);

        let mut beta = make_test_msg("Beta");
        add_msg_field(&mut beta, 1, "gamma", ".pkg.Gamma", false, None);
        add_msg_field(&mut beta, 2, "omega", ".pkg.Omega", true, Some(None));

        let mut gamma = make_test_msg("Gamma");
        add_msg_field(&mut gamma, 1, "alpha", ".pkg.Alpha", false, None);

        let mut omega = make_test_msg("Omega");
        add_msg_field(&mut omega, 1, "alpha", ".pkg.Alpha", false, None);

        // self-referential
        let mut sigma = make_test_msg("Sigma");
        add_msg_field(&mut sigma, 1, "sigma", ".pkg.Sigma", false, None);

        let mut graph = TypeGraph::default();
        graph.add_message(".pkg.Alpha".to_owned(), alpha);
        graph.add_message(".pkg.Beta".to_owned(), beta);
        graph.add_message(".pkg.Gamma".to_owned(), gamma);
        graph.add_message(".pkg.Omega".to_owned(), omega);
        graph.add_message(".pkg.Sigma".to_owned(), sigma);

        graph.box_cyclic_dependencies();
        graph.max_size_cyclic_dependencies();

        let alpha = graph.get_message(".pkg.Alpha").unwrap();
        assert!(!alpha.fields[0].boxed);
        assert_eq!(alpha.fields[0].max_size_override, None);
        let beta = graph.get_message(".pkg.Beta").unwrap();
        assert!(!beta.fields[0].boxed);
        assert_eq!(beta.fields[0].max_size_override, None);
        assert!(beta.fields[1].boxed);
        assert_eq!(beta.fields[1].max_size_override, Some(None));
        let gamma = graph.get_message(".pkg.Gamma").unwrap();
        assert!(gamma.fields[0].boxed); // Gamma.alpha should have been boxed
        assert_eq!(gamma.fields[0].max_size_override, Some(None));
        let omega = graph.get_message(".pkg.Omega").unwrap();
        assert!(!omega.fields[0].boxed); // Omega.alpha should stay unboxed, since Beta.omega was already boxed
        assert_eq!(omega.fields[0].max_size_override, None);
        let sigma = graph.get_message(".pkg.Sigma").unwrap();
        assert!(sigma.fields[0].boxed); // Sigma.sigma should have been boxed
        assert_eq!(sigma.fields[0].max_size_override, Some(None));
    }
}
