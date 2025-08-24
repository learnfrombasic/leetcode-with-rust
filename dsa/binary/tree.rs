#[derive(Debug)]
struct Node {
    key: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32) -> Self {
        Node { key, left: None, right: None }
    }

    fn insert(&mut self, key: i32) {
        if key < self.key {
            match self.left {
                Some(ref mut left) => left.insert(key),
                None => self.left = Some(Box::new(Node::new(key))),
            }
        } else if key > self.key {
            match self.right {
                Some(ref mut right) => right.insert(key),
                None => self.right = Some(Box::new(Node::new(key))),
            }
        }
    }

    fn search(&self, key: i32) -> bool {
        if key == self.key {
            true
        } else if key < self.key {
            self.left.as_ref().map_or(false, |left| left.search(key))
        } else {
            self.right.as_ref().map_or(false, |right| right.search(key))
        }
    }

    fn find_min(&self) -> i32 {
        match &self.left {
            Some(left) => left.find_min(),
            None => self.key,
        }
    }

    fn delete(self: Box<Self>, key: i32) -> Option<Box<Node>> {
        if key < self.key {
            Some(Box::new(Node {
                key: self.key,
                left: self.left.and_then(|left| left.delete(key)),
                right: self.right,
            }))
        } else if key > self.key {
            Some(Box::new(Node {
                key: self.key,
                left: self.left,
                right: self.right.and_then(|right| right.delete(key)),
            }))
        } else {
            match (self.left, self.right) {
                (None, None) => None,
                (Some(child), None) | (None, Some(child)) => Some(child),
                (Some(left), Some(right)) => {
                    let min_key = right.find_min();
                    Some(Box::new(Node {
                        key: min_key,
                        left: Some(left),
                        right: right.delete(min_key),
                    }))
                }
            }
        }
    }
}
