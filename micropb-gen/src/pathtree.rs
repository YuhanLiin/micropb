pub struct Node<T> {
    value: Option<T>,
    children: Vec<(String, Node<T>)>,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            children: Default::default(),
        }
    }
}

impl<T> Node<T> {
    pub fn next(&self, segment: &str) -> Option<&Self> {
        self.children
            .iter()
            .find(|(c, _)| c == segment)
            .map(|(_, next)| next)
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn value_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }

    pub fn add<'a>(&mut self, path: impl Iterator<Item = &'a str>, value: T) -> &mut Node<T> {
        let mut node = self;
        for segment in path {
            if let Some(pos) = node.children.iter().position(|(c, _)| c == segment) {
                node = &mut node.children[pos].1;
            } else {
                node.children.push((segment.to_owned(), Default::default()));
                node = &mut node.children.last_mut().unwrap().1;
            }
        }
        node.value = Some(value);
        node
    }

    pub fn get<'a>(
        &self,
        path: impl Iterator<Item = &'a str>,
        mut callback: impl FnMut(&T),
    ) -> Option<&Node<T>> {
        let mut node = self;
        for segment in path {
            node.value.as_ref().map(&mut callback);

            if let Some(next) = node.next(segment) {
                node = next;
            } else {
                return None;
            }
        }
        Some(node)
    }
}

pub struct PathTree<T> {
    pub root: Node<T>,
}

impl<T> PathTree<T> {
    pub fn new(value: T) -> Self {
        Self {
            root: Node {
                value: Some(value),
                children: vec![],
            },
        }
    }
}
