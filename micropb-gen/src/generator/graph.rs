use std::collections::{BTreeMap, BTreeSet};

use crate::generator::{r#enum::Enum, field::FieldType, message::Message, oneof::Oneof};

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
            Position::Oneof(oi, fi) => {
                let oneof = &mut msg.oneofs[*oi];
                // If the oneof itself is boxed, then return None since there's no need for cycle
                // detection
                let not_boxed = !oneof.boxed;
                not_boxed.then(|| {
                    &mut oneof.otype.fields_mut().expect("unexpected custom oneof")[*fi].boxed
                })
            }
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
pub(crate) struct TypeGraph<'a> {
    messages: BTreeMap<String, Message<'a>>,
    enums: BTreeMap<String, Enum<'a>>,
}

impl<'a> TypeGraph<'a> {
    pub(crate) fn add_message(&mut self, fq_proto_name: String, msg: Message<'a>) {
        self.messages.insert(fq_proto_name, msg);
    }

    pub(crate) fn add_enum(&mut self, fq_proto_name: String, e: Enum<'a>) {
        self.enums.insert(fq_proto_name, e);
    }

    pub(crate) fn get_message(&self, fq_proto_name: &str) -> Option<&Message<'a>> {
        self.messages.get(fq_proto_name)
    }

    pub(crate) fn get_enum(&self, fq_proto_name: &str) -> Option<&Enum<'a>> {
        self.enums.get(fq_proto_name)
    }

    fn populate_parents(&mut self) {
        let names: Vec<_> = self.messages.keys().cloned().collect();
        for name in &names {
            let msg = self.messages.get_mut(name).unwrap();
            let edges = msg.message_edges.clone();
            for (pos, next_name) in edges {
                if let Some(next_msg) = self.messages.get_mut(next_name) {
                    next_msg.parent_edges.push((pos, name.clone()));
                }
            }
        }
    }

    fn reverse_propagate(
        &mut self,
        starting_elems: Vec<RevElem>,
        mark_msg: impl Fn(&mut Message, &RevElem),
    ) -> BTreeSet<RevElem> {
        let mut elems = starting_elems;
        let mut visited = BTreeSet::new();

        while let Some(elem) = elems.pop() {
            if visited.contains(&elem) {
                continue;
            }

            let msg_name = elem.name();
            let Some(cur_msg) = self.messages.get_mut(msg_name) else {
                continue;
            };
            mark_msg(cur_msg, &elem);

            match &elem {
                RevElem::Oneof(name, _) => elems.push(RevElem::Msg(name.clone())),
                RevElem::Msg(_) => {
                    for (pos, parent) in cur_msg.parent_edges.iter() {
                        let next_elem = match pos {
                            Position::Field(_) => RevElem::Msg(parent.clone()),
                            Position::Oneof(oneof_idx, _) => {
                                RevElem::Oneof(parent.clone(), *oneof_idx)
                            }
                        };
                        elems.push(next_elem);
                    }
                }
            }

            visited.insert(elem);
        }
        visited
    }

    fn propagate_lifetimes(&mut self) {
        let mut lifetime = None;
        let msgs_with_lifetime = self
            .messages
            .iter()
            .filter(|(_, msg)| {
                if lifetime.is_none() {
                    lifetime = msg.lifetime.clone();
                }
                msg.lifetime.is_some()
            })
            .map(|(name, _)| RevElem::Msg(name.clone()))
            .collect();

        self.reverse_propagate(msgs_with_lifetime, |msg, elem| match elem {
            RevElem::Msg(_) => msg.lifetime = lifetime.clone(),
            RevElem::Oneof(_, idx) => msg.oneofs[*idx].lifetime = lifetime.clone(),
        });
    }

    /// Propagate the falseness of a boolean flag up the graph
    fn propagate_bool_false(
        &mut self,
        get_msg: impl Fn(&Message) -> bool,
        get_oneof: impl Fn(&Oneof) -> bool,
        set_msg: impl Fn(&mut Message, bool),
        set_oneof: impl Fn(&mut Oneof, bool),
    ) {
        let starting_msgs = self
            .messages
            .iter()
            .filter(|(_, msg)| !get_msg(msg))
            .map(|(name, _)| RevElem::Msg(name.clone()));
        let starting_oneofs = self
            .messages
            .iter()
            .flat_map(|(name, msg)| {
                msg.oneofs
                    .iter()
                    .enumerate()
                    .map(move |(i, oneof)| (name, i, oneof))
            })
            .filter(|(_, _, oneof)| !get_oneof(oneof))
            .map(|(name, i, _)| RevElem::Oneof(name.clone(), i));
        let starting_elems = starting_msgs.chain(starting_oneofs).collect();

        self.reverse_propagate(starting_elems, |msg, elem| match elem {
            RevElem::Msg(_) => set_msg(msg, false),
            RevElem::Oneof(_, idx) => set_oneof(&mut msg.oneofs[*idx], false),
        });
    }

    fn propagate_no_dbg(&mut self) {
        self.propagate_bool_false(
            |msg| msg.derive_dbg,
            |oneof| oneof.derive_dbg,
            |msg, b| msg.derive_dbg = b,
            |oneof, b| oneof.derive_dbg = b,
        );
    }

    fn propagate_no_clone(&mut self) {
        self.propagate_bool_false(
            |msg| msg.derive_clone,
            |oneof| oneof.derive_clone,
            |msg, b| msg.derive_clone = b,
            |oneof, b| oneof.derive_clone = b,
        );
    }

    fn propagate_no_partial_eq(&mut self) {
        self.propagate_bool_false(
            |msg| msg.impl_partial_eq,
            |oneof| oneof.derive_partial_eq,
            |msg, b| msg.impl_partial_eq = b,
            |oneof, b| oneof.derive_partial_eq = b,
        );
    }

    // Reverse propagating no-default for repeated and map fields is incorrect, because those
    // fields don't need `T: Default` to be Default. This is a bothersome corner case, so I'll just
    // leave it unimplemented
    //fn propagate_no_default(&mut self) {
    //self.propagate_bool_false(
    //|msg| msg.impl_default,
    //// Oneof will always implement default
    //|__| true,
    //|msg, b| msg.impl_default = b,
    //|_, _| {},
    //);
    //}

    /// Forward DFS that performs conditional cycle detection. Does not care about oneofs.
    fn forward_dfs<'b>(
        &mut self,
        start: &'b [String],
        pursue_edge: impl Fn(&Position, &mut Message) -> bool,
        break_cycle: impl Fn(&Position, &mut Message),
        msg_finish: impl Fn(&mut Self, &str),
    ) where
        'a: 'b,
    {
        let mut edges: Vec<_> = start.into_iter().map(|m| DfsElem::Edge(m)).collect();
        let mut ancestors = BTreeSet::new();
        let mut visited = BTreeSet::new();

        while let Some(elem) = edges.pop() {
            match elem {
                DfsElem::Edge(cur_field) => {
                    if visited.contains(cur_field) {
                        continue;
                    }
                    visited.insert(cur_field);

                    let Some(cur_msg) = self.messages.get_mut(cur_field) else {
                        continue;
                    };

                    ancestors.insert(cur_field);
                    edges.push(DfsElem::NodeEnd(cur_field));

                    for i in 0..cur_msg.message_edges.len() {
                        let (pos, next_field) = cur_msg.message_edges[i];
                        if pursue_edge(&pos, cur_msg) {
                            if ancestors.contains(next_field) {
                                break_cycle(&pos, cur_msg);
                            } else {
                                edges.push(DfsElem::Edge(next_field));
                            }
                        }
                    }
                }

                DfsElem::NodeEnd(msg) => {
                    msg_finish(self, msg);
                    ancestors.remove(msg);
                }
            }
        }
    }

    /// Detect cycles in the message graph via DFS and break those cycles by boxing fields.
    fn box_cyclic_dependencies(&mut self) {
        let messages: Vec<_> = self.messages.keys().cloned().collect();

        self.forward_dfs(
            &messages,
            |pos, msg| pos.is_boxed_mut(msg) == Some(&mut false),
            |pos, msg| *pos.is_boxed_mut(msg).unwrap() = true,
            |_, _| {},
        );
    }

    /// Detect cycles in the message graph via DFS and break those cycles by overriding max size
    fn max_size_cyclic_dependencies(&mut self) {
        let messages: Vec<_> = self.messages.keys().cloned().collect();

        self.forward_dfs(
            &messages,
            // Pursue fields where max size isn't overridden
            |pos, msg| matches!(pos.max_size_override_mut(msg), Some(None)),
            // Break the cycle by setting MAX_SIZE to None, resulting in the MAX_SIZE of all
            // messages in the cycle to become None
            |pos, msg| *pos.max_size_override_mut(msg).unwrap() = Some(None),
            |_, _| {},
        );
    }

    fn propagate_derive_copy(&mut self) {
        let messages: Vec<_> = self.messages.keys().cloned().collect();

        self.forward_dfs(
            &messages,
            |_, _| true,
            |_, _| {},
            |this, msg_name| {
                // msg_name must be present in the graph, otherwise it wouldn't have been processed by
                // the DFS
                let msg = this.get_message(msg_name).unwrap();
                // Check sub-messages
                let sub_msgs_copy = msg
                    .message_edges
                    .iter()
                    .all(|(_, child)| this.get_message(child).map(|c| c.is_copy).unwrap_or(false));

                let msg = this.messages.get_mut(msg_name).unwrap();
                msg.check_is_copy(sub_msgs_copy);
            },
        );
    }

    pub(crate) fn resolve_all(&mut self) {
        // Generate parent edges for all messages
        self.populate_parents();

        // Reverse propagation
        self.propagate_lifetimes();
        self.propagate_no_dbg();
        self.propagate_no_clone();
        self.propagate_no_partial_eq();
        //self.propagate_no_default();

        // Cyclic dependencies
        self.box_cyclic_dependencies();
        self.max_size_cyclic_dependencies();

        self.propagate_derive_copy();
    }
}

/// Represents either a message or a oneof. Used for reverse propagation.
#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum RevElem {
    Msg(String),
    Oneof(String, usize),
}

impl RevElem {
    fn name(&self) -> &str {
        match self {
            RevElem::Msg(name) => name,
            RevElem::Oneof(name, _) => name,
        }
    }
}

/// Used for cycle detection
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
            oneof::{make_test_oneof, make_test_oneof_field},
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
            FieldType::Optional(TypeSpec::Message(type_name), OptionalRepr::Option),
        );
        field.max_size_override = max_size_override;
        msg.fields.push(field);
    }

    fn add_oneof_field<'a>(
        msg: &mut Message<'a>,
        oneof_idx: usize,
        num: u32,
        fname: &'a str,
        type_name: &'a str,
        boxed: bool,
        max_size_override: Option<Option<usize>>,
    ) {
        let oneof_fields = msg.oneofs[0].otype.fields_mut().unwrap();
        msg.message_edges
            .push((Position::Oneof(oneof_idx, oneof_fields.len()), type_name));
        let mut field = make_test_oneof_field(num, fname, boxed, TypeSpec::Message(type_name));
        field.max_size_override = max_size_override;
        oneof_fields.push(field);
    }

    #[test]
    fn cyclic_dependencies() {
        //    <- G <--
        //   /       |
        //  A -----> B     S <-> S
        //   \       |
        //    <- O <--
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

        // Verification
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

    #[test]
    fn reverse_propagate() {
        //     <---------------------
        //    /        --0           \
        //   /        /               \
        //  A* ----> B --1--> G* ----> O    S --0*
        //   \               /  \
        //    -------------->    ----> T
        let mut alpha = make_test_msg("Alpha");
        alpha.derive_dbg = false;
        add_msg_field(&mut alpha, 1, "beta", ".pkg.Beta", false, None);
        add_msg_field(&mut alpha, 2, "gamma", ".pkg.Gamma", false, None);

        let mut beta = make_test_msg("Beta");
        add_msg_field(&mut beta, 1, "gamma", ".pkg.Gamma", false, None);
        beta.oneofs.push(make_test_oneof("empty", false));
        // Create oneof with a single field
        beta.oneofs.push(make_test_oneof("omega", false));
        add_oneof_field(&mut beta, 1, 2, "omega", ".pkg.Omega", false, None);

        let mut gamma = make_test_msg("Gamma");
        gamma.derive_dbg = false;
        add_msg_field(&mut gamma, 1, "theta", ".pkg.Theta", false, None);

        let mut omega = make_test_msg("Omega");
        add_msg_field(&mut omega, 1, "alpha", ".pkg.Alpha", false, None);

        let theta = make_test_msg("Theta");

        let mut sigma = make_test_msg("Sigma");
        sigma.oneofs.push(make_test_oneof("sigma", false));
        sigma.oneofs[0].derive_dbg = false; // Sigma has debug, but the oneof doesn't

        let mut graph = TypeGraph::default();
        graph.add_message(".pkg.Alpha".to_owned(), alpha);
        graph.add_message(".pkg.Beta".to_owned(), beta);
        graph.add_message(".pkg.Gamma".to_owned(), gamma);
        graph.add_message(".pkg.Omega".to_owned(), omega);
        graph.add_message(".pkg.Theta".to_owned(), theta);
        graph.add_message(".pkg.Sigma".to_owned(), sigma);

        graph.populate_parents();
        graph.propagate_no_dbg();

        // Verification
        // Expect all entities to have no debug except for Theta and Beta.oneofs[0]
        let alpha = graph.get_message(".pkg.Alpha").unwrap();
        assert!(!alpha.derive_dbg);
        assert_eq!(
            alpha.parent_edges,
            vec![(Position::Field(0), ".pkg.Omega".to_owned())]
        );

        let beta = graph.get_message(".pkg.Beta").unwrap();
        assert!(!beta.derive_dbg);
        assert_eq!(
            beta.parent_edges,
            vec![(Position::Field(0), ".pkg.Alpha".to_owned())]
        );
        assert!(beta.oneofs[0].derive_dbg);
        assert!(!beta.oneofs[1].derive_dbg);

        let gamma = graph.get_message(".pkg.Gamma").unwrap();
        assert!(!gamma.derive_dbg);
        assert_eq!(
            gamma.parent_edges,
            vec![
                (Position::Field(1), ".pkg.Alpha".to_owned()),
                (Position::Field(0), ".pkg.Beta".to_owned())
            ]
        );

        let omega = graph.get_message(".pkg.Omega").unwrap();
        assert!(!omega.derive_dbg);
        assert_eq!(
            omega.parent_edges,
            vec![(Position::Oneof(1, 0), ".pkg.Beta".to_owned()),]
        );

        let theta = graph.get_message(".pkg.Theta").unwrap();
        assert!(theta.derive_dbg);
        assert_eq!(
            theta.parent_edges,
            vec![(Position::Field(0), ".pkg.Gamma".to_owned())]
        );

        let sigma = graph.get_message(".pkg.Sigma").unwrap();
        assert!(!sigma.derive_dbg);
        assert!(!sigma.oneofs[0].derive_dbg);
        assert_eq!(sigma.parent_edges, vec![])
    }
}
