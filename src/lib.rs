use std::cmp::Ordering;

#[derive(Debug)]
pub struct BinarySearchTree<T: Ord> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(
        &mut self,
        value: T,
    ) {
        match &mut self.root {
            Some(tree) => {
                tree.insert_unique(value);
            },
            None => {
                self.root = Some(Box::new(Node::new(value)));
            },
        }
        self.size += 1;
    }

    pub fn min(&self) -> Option<&T> {
        match &self.root {
            Some(tree) => Some(tree.min()),
            None => None,
        }
    }

    pub fn max(&self) -> Option<&T> {
        match &self.root {
            Some(tree) => Some(tree.max()),
            None => None,
        }
    }

    pub fn contains(
        &self,
        value: T,
    ) -> bool {
        match &self.root {
            Some(tree) => tree.contains(value),
            None => false,
        }
    }

    pub fn pop_min(&self) -> Option<T> {
        todo!();
    }

    pub fn pop_max(&self) -> Option<T> {
        todo!();
    }
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    count: usize,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
            count: 0,
        }
    }

    fn insert_unique(
        &mut self,
        value: T,
    ) -> Option<()> {
        let mut current = self;

        loop {
            let sub = match value.cmp(&current.value) {
                Ordering::Less => &mut current.left,
                Ordering::Greater => &mut current.right,
                Ordering::Equal => {
                    current.count += 1;
                    return None;
                },
            };

            match sub {
                None => {
                    *sub = Some(Box::new(Self::new(value)));
                    return Some(());
                },
                Some(boxed_tree) => {
                    current = boxed_tree;
                },
            }
        }
    }

    fn min(&self) -> &T {
        let mut min = self;
        while min.left.is_some() {
            min = min.left.as_ref().unwrap();
        }
        &min.value
    }

    fn max(&self) -> &T {
        let mut max = self;
        while max.right.is_some() {
            max = max.right.as_ref().unwrap();
        }
        &max.value
    }

    fn contains(
        &self,
        value: T,
    ) -> bool {
        let mut current = self;

        loop {
            let sub = match value.cmp(&current.value) {
                Ordering::Less => &current.left,
                Ordering::Greater => &current.right,
                Ordering::Equal => {
                    return true;
                },
            };
            match sub {
                None => {
                    return false;
                },
                Some(boxed_tree) => {
                    current = boxed_tree;
                },
            }
        }
    }

    // fn delete_recursive(
    //     mut self,
    //     value: &T,
    // ) -> Option<Box<Node<T>>> {
    //     match value.cmp(&self.value) {
    //         Ordering::Less => {
    //             self.left = self.left?.delete_recursive(value);
    //             Some(Box::new(self))
    //         },
    //         Ordering::Greater => {
    //             self.right = self.right?.delete_recursive(value);
    //             Some(Box::new(self))
    //         },
    //         Ordering::Equal => None,
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tree() -> BinarySearchTree<i32> {
        let mut tree = BinarySearchTree::new();
        tree.insert(25);
        tree.insert(15);
        tree.insert(40);
        tree.insert(10);
        tree.insert(18);
        tree.insert(45);
        tree.insert(35);
        tree
    }

    #[test]
    fn insert() {
        let tree = make_tree();
        assert!(tree.root.as_ref().unwrap().value == 25);

        assert!(tree.root.as_ref().unwrap().left.as_ref().unwrap().value == 15);
        assert!(
            tree.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .value
                == 10
        );
        assert!(
            tree.root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .value
                == 18
        );

        assert!(tree.root.as_ref().unwrap().right.as_ref().unwrap().value == 40);
        assert!(
            tree.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .value
                == 35
        );
        assert!(
            tree.root
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .value
                == 45
        );
    }

    #[test]
    fn min() {
        let tree = make_tree();
        assert!(tree.min() == Some(&10));
    }

    #[test]
    fn max() {
        let tree = make_tree();
        assert!(tree.max() == Some(&45));
    }

    #[test]
    fn contains() {
        let tree = make_tree();
        assert!(tree.contains(25));
        assert!(tree.contains(15));
        assert!(tree.contains(10));
        assert!(tree.contains(15));
        assert!(tree.contains(40));
        assert!(tree.contains(35));
        assert!(tree.contains(45));

        assert!(!tree.contains(5));
        assert!(!tree.contains(46));
        assert!(!tree.contains(17))
    }
}
