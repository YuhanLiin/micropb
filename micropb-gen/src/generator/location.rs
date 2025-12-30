use std::ops::Deref;

use crate::{
    descriptor::SourceCodeInfo_::Location,
    pathtree::{Node, PathTree},
};

#[derive(Debug, Default)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct Comments {
    leading: Vec<String>,
    trailing: Vec<String>,
}

impl Comments {
    /// Return None if there aren't any comments we care about
    fn from_location(location: &Location) -> Option<Self> {
        let leading = location
            .leading_comments()
            .map_or_else(Default::default, |c| get_lines(c));
        let trailing = location
            .trailing_comments()
            .map_or_else(Default::default, |c| get_lines(c));
        if leading.is_empty() && trailing.is_empty() {
            return None;
        }
        Some(Self { leading, trailing })
    }

    pub(crate) fn lines(&self) -> impl Iterator<Item = &str> {
        let leading_and_trailing = !self.leading.is_empty() && !self.trailing.is_empty();
        // Insert empty line between leading and trailing comments if both are present
        let empty_line = leading_and_trailing.then_some("").into_iter();
        self.leading
            .iter()
            .map(Deref::deref)
            .chain(empty_line)
            .chain(self.trailing.iter().map(Deref::deref))
    }
}

fn get_lines(comment: &str) -> Vec<String> {
    comment.lines().map(ToOwned::to_owned).collect()
}

pub(crate) type CommentNode = Node<Comments, (i32, i32)>;

/// Each Location has a path and maybe some comments. The path is a list of numbers that point to
/// some item in the proto file. The numbers in the path can be grouped into pairs, where each pair
/// is an "edge" in the FileDescriptorSet hierarchy. The first number in the pair is the field
/// number of the descriptor, and the second number is the index within that field.
///
/// The natural way to process paths is to split them into pairs and use them as edges in the
/// PathTree, with the comments as the nodes. That way the shape of the comment PathTree matches
/// the shape of the config tree, so both trees can be walked together as we walk the
/// FileDescriptorSet structure.
pub(crate) fn add_location_comments(
    tree: &mut PathTree<Comments, (i32, i32)>,
    location: &Location,
) {
    // If the location has no comments, skip this step
    let Some(comments) = Comments::from_location(location) else {
        return;
    };

    let path = &location.path;
    if !path.len().is_multiple_of(2) {
        return;
    }
    let segments: Vec<_> = path.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    *tree.root.add_path(segments.iter()).value_mut() = Some(comments);
}

pub(crate) fn next_comment_node(
    node: Option<&CommentNode>,
    segment: (i32, i32),
) -> Option<&CommentNode> {
    node.and_then(|n| n.next(&segment))
}

pub(crate) fn get_comments(node: Option<&CommentNode>) -> Option<&Comments> {
    node.and_then(|n| n.access_value().as_ref())
}

pub(crate) mod path {
    pub(crate) fn fdset_msg(idx: usize) -> (i32, i32) {
        (4, idx as i32)
    }

    pub(crate) fn fdset_enum(idx: usize) -> (i32, i32) {
        (5, idx as i32)
    }

    pub(crate) fn msg_field(idx: usize) -> (i32, i32) {
        (2, idx as i32)
    }

    pub(crate) fn msg_msg(idx: usize) -> (i32, i32) {
        (3, idx as i32)
    }

    pub(crate) fn msg_enum(idx: usize) -> (i32, i32) {
        (4, idx as i32)
    }

    pub(crate) fn msg_oneof(idx: usize) -> (i32, i32) {
        (8, idx as i32)
    }

    pub(crate) fn enum_value(idx: usize) -> (i32, i32) {
        (2, idx as i32)
    }
}
