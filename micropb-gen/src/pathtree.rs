use std::{borrow::Borrow, cell::Cell};

#[derive(Debug)]
pub struct Node<T, S: ?Sized + ToOwned = str> {
    value: Option<T>,
    children: Vec<(S::Owned, Node<T, S>)>,
    accessed: Cell<bool>,
}

impl<T, S: ?Sized + ToOwned> Default for Node<T, S> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            children: Default::default(),
            accessed: Default::default(),
        }
    }
}

impl<T, S: ?Sized + ToOwned + PartialEq> Node<T, S>
where
    S::Owned: Default,
{
    pub fn next(&self, segment: &S) -> Option<&Self> {
        self.children
            .iter()
            .find(|(c, _)| c.borrow() == segment)
            .map(|(_, next)| next)
    }

    pub fn access_value(&self) -> &Option<T> {
        self.accessed.set(true);
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut Option<T> {
        &mut self.value
    }

    pub fn add_path<'a>(&mut self, path: impl Iterator<Item = &'a S>) -> &mut Node<T, S>
    where
        S: 'a,
    {
        let mut node = self;
        for segment in path {
            if let Some(pos) = node
                .children
                .iter()
                .position(|(c, _)| c.borrow() == segment)
            {
                node = &mut node.children[pos].1;
            } else {
                node.children.push((segment.to_owned(), Default::default()));
                node = &mut node.children.last_mut().unwrap().1;
            }
        }
        node
    }

    pub fn visit_path<'a>(
        &self,
        path: impl Iterator<Item = &'a S>,
        mut callback: impl FnMut(&T),
    ) -> Option<&Node<T, S>>
    where
        S: 'a,
    {
        let mut node = self;
        for segment in path {
            if let Some(next) = node.next(segment) {
                next.access_value().as_ref().map(&mut callback);
                node = next;
            } else {
                return None;
            }
        }
        Some(node)
    }

    pub fn children_mut(&mut self) -> impl Iterator<Item = &mut (S::Owned, Node<T, S>)> {
        self.children.iter_mut()
    }
}

pub struct PathTree<T, S: ?Sized + ToOwned = str> {
    pub root: Node<T, S>,
}

impl<T, S: ?Sized + ToOwned> PathTree<T, S> {
    pub fn new(value: T) -> Self {
        Self {
            root: Node {
                value: Some(value),
                children: vec![],
                accessed: Cell::new(true), // root will always be accessed
            },
        }
    }
}

impl<T> PathTree<T> {
    pub fn find_all_unaccessed(&self, mut reporter: impl FnMut(&Node<T>, &[&str])) {
        let mut edges = vec![DfsElem::Edge(("", &self.root))];
        let mut path = vec![];
        while let Some(elem) = edges.pop() {
            if let DfsElem::Edge((next_part, cur_node)) = elem {
                path.push(next_part);
                if cur_node.value.is_some() && !cur_node.accessed.get() {
                    reporter(cur_node, &path);
                }
                edges.push(DfsElem::NodeEnd);
                for (part, node) in &cur_node.children {
                    edges.push(DfsElem::Edge((part, node)));
                }
            } else {
                // NodeEnd
                path.pop();
            }
        }
    }
}

enum DfsElem<T> {
    Edge(T),
    NodeEnd,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn straight_path() {
        let mut root = Node::default();
        root.add_path(["a", "b", "c"].into_iter()).value = Some(5);

        assert_eq!(root.value, None);
        assert_eq!(root.children.len(), 1);
        assert!(root.next("ab").is_none());
        assert!(root.next("b").is_none());

        let node = root.next("a").unwrap();
        assert_eq!(root.value, None);
        assert_eq!(root.children.len(), 1);
        let node = node.next("b").unwrap();
        assert_eq!(node.value, None);
        assert_eq!(node.children.len(), 1);
        let node = node.next("c").unwrap();
        assert_eq!(node.value, Some(5));
        assert!(node.children.is_empty());

        root.add_path(["a", "b"].into_iter()).value = Some(3);
        let node = root.next("a").unwrap();
        assert_eq!(root.value, None);
        assert_eq!(root.children.len(), 1);
        let node = node.next("b").unwrap();
        assert_eq!(node.value, Some(3));
        assert_eq!(node.children.len(), 1);

        let mut total = 0;
        let node = root
            .visit_path(["a", "b", "c"].into_iter(), |i| total += i)
            .unwrap();
        assert_eq!(total, 8);
        assert_eq!(node.value, Some(5));

        let mut total = 0;
        assert!(root
            .visit_path(["a", "c", "c"].into_iter(), |i| total += i)
            .is_none());
        assert_eq!(total, 0);
    }

    fn assert_visit_path(root: &Node<char>, path: &[&str], expected: &str) -> Option<char> {
        let mut s = String::new();
        let node = root.visit_path(path.iter().copied(), |&c| s.push(c));
        assert_eq!(s, expected);
        node.and_then(|n| n.value)
    }

    #[test]
    fn multiple_paths() {
        let mut root = Node::default();
        root.add_path(["fruit", "apple"].into_iter()).value = Some('a');
        root.add_path(["fruit", "orange"].into_iter()).value = Some('o');
        root.add_path(["fruit", "pear"].into_iter()).value = Some('p');
        root.add_path(["fruit"].into_iter()).value = Some('f');
        root.add_path(["car"].into_iter()).value = Some('c');

        assert_eq!(assert_visit_path(&root, &["car"], "c"), Some('c'));
        assert_eq!(assert_visit_path(&root, &["fruit"], "f"), Some('f'));
        assert_eq!(
            assert_visit_path(&root, &["fruit", "apple"], "fa"),
            Some('a')
        );
        assert_eq!(
            assert_visit_path(&root, &["fruit", "orange"], "fo"),
            Some('o')
        );
        assert_eq!(
            assert_visit_path(&root, &["fruit", "pear"], "fp"),
            Some('p')
        );
        assert_eq!(assert_visit_path(&root, &["car", "salesman"], "c"), None);
        assert_eq!(assert_visit_path(&root, &["fruit", "salesman"], "f"), None);
        assert_eq!(assert_visit_path(&root, &["alien"], ""), None);
    }

    fn get_unaccessed_paths<T>(tree: &PathTree<T>) -> Vec<String> {
        let mut paths = vec![];
        tree.find_all_unaccessed(|_node, path| paths.push(path.join(".")));
        paths
    }

    #[test]
    fn find_all_unaccessed() {
        let mut tree = PathTree::new('0');
        tree.root.add_path(["fruit", "apple"].into_iter()).value = Some('a');
        tree.root.add_path(["fruit", "orange"].into_iter()).value = Some('o');
        tree.root.add_path(["fruit", "pear"].into_iter()).value = Some('p');
        tree.root.add_path(["animal", "cat"].into_iter()).value = Some('c');
        tree.root.add_path(["fruit"].into_iter()).value = Some('f');
        tree.root.add_path(["car"].into_iter()).value = Some('c');

        tree.root
            .next("fruit")
            .unwrap()
            .next("apple")
            .unwrap()
            .accessed
            .set(true);

        let unaccessed = get_unaccessed_paths(&tree);
        assert_eq!(
            unaccessed,
            &[
                ".car",
                ".animal.cat",
                ".fruit",
                ".fruit.pear",
                ".fruit.orange",
            ]
        );
    }
}
