use crate::{
    descriptor::SourceCodeInfo_::Location,
    pathtree::{Node, PathTree},
};

#[derive(Default)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub(crate) struct Comments {
    leading: Vec<String>,
    trailing: Vec<String>,
}

impl Comments {
    fn from_location(location: &Location) -> Option<Self> {
        let (Some(leading_comments), Some(trailing_comments)) =
            (location.leading_comments(), location.trailing_comments())
        else {
            return None;
        };
        Some(Self {
            leading: get_lines(leading_comments),
            trailing: get_lines(trailing_comments),
        })
    }

    pub(crate) fn lines(&self) -> impl Iterator<Item = &String> {
        self.leading.iter().chain(self.trailing.iter())
    }
}

fn get_lines(comment: &String) -> Vec<String> {
    // TODO sanitize
    comment.lines().map(ToOwned::to_owned).collect()
}

pub(crate) type CommentNode = Node<Comments, (i32, i32)>;

pub(crate) fn add_location_comments(
    tree: &mut PathTree<Comments, (i32, i32)>,
    location: &Location,
) {
    // If the location has no comments, skip this step
    let Some(comments) = Comments::from_location(location) else {
        return;
    };

    let path = &location.path;
    if path.len() % 2 != 0 {
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
