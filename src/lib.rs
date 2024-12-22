use std::cmp::Ordering;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct BinarySearchTree<T: Ord> {
    root: Tree<T>,
    size: usize,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self {
            root: Tree(None),
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
    ) -> Option<()> {
        if self.root.insert_unique(value).is_some() {
            self.size += 1;
            Some(())
        } else {
            None
        }
    }

    pub fn min(&self) -> Option<&T> {
        self.root.min()
    }

    pub fn max(&self) -> Option<&T> {
        self.root.max()
    }

    pub fn contains(
        &self,
        value: &T,
    ) -> bool {
        self.root.contains(value)
    }

    pub fn pop_min(&mut self) -> Option<T> {
        self.root.pop_min()
    }

    pub fn pop_max(&mut self) -> Option<T> {
        self.root.pop_max()
    }

    pub fn remove(
        &mut self,
        value: &T,
    ) -> Option<()> {
        self.root.remove(value).map(|_| self.size -= 1)
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
    left: Tree<T>,
    right: Tree<T>,
}

#[derive(Debug)]
struct Tree<T: Ord>(Option<Box<Node<T>>>);

impl<T: Ord> Tree<T> {
    fn new() -> Self {
        Self(None)
    }

    fn insert_unique(
        &mut self,
        value: T,
    ) -> Option<()> {
        let mut current = self;

        while let Some(ref mut node) = current.0 {
            current = match value.cmp(&node.value) {
                Ordering::Less => &mut node.left,
                Ordering::Greater => &mut node.right,
                Ordering::Equal => {
                    return None;
                },
            };
        }
        current.0 = Some(Box::new(Node {
            value,
            left: Self::new(),
            right: Self::new(),
        }));
        Some(())
    }

    fn min(&self) -> Option<&T> {
        let min = self;
        match min {
            Tree(None) => None,
            Tree(Some(boxed_node)) => {
                let mut boxed_node = boxed_node;

                while let Tree(Some(left)) = &boxed_node.left {
                    boxed_node = left;
                }
                Some(&boxed_node.value)
            },
        }
    }

    fn pop_min(&mut self) -> Option<T> {
        let mut min_tree = self;
        match min_tree {
            Tree(None) => None,
            Tree(Some(_)) => {
                while let Tree(Some(_)) = min_tree.0.as_mut().unwrap().left {
                    min_tree = &mut min_tree.0.as_mut().unwrap().left;
                }
                let value = min_tree.0.take().unwrap().value;
                Some(value)
            },
        }
    }

    fn pop_max(&mut self) -> Option<T> {
        let mut max_tree = self;
        match max_tree {
            Tree(None) => None,
            Tree(Some(_)) => {
                while let Tree(Some(_)) = max_tree.0.as_mut().unwrap().right {
                    max_tree = &mut max_tree.0.as_mut().unwrap().right;
                }
                let value = max_tree.0.take().unwrap().value;
                Some(value)
            },
        }
    }

    fn max(&self) -> Option<&T> {
        let max = self;
        match max {
            Tree(None) => None,
            Tree(Some(boxed_node)) => {
                let mut boxed_node = boxed_node;

                while let Tree(Some(right)) = &boxed_node.right {
                    boxed_node = right;
                }
                Some(&boxed_node.value)
            },
        }
    }

    fn contains(
        &self,
        value: &T,
    ) -> bool {
        let mut current = self;

        while let Tree(Some(boxed_node)) = current {
            let sub = match value.cmp(&boxed_node.value) {
                Ordering::Less => &boxed_node.left,
                Ordering::Greater => &boxed_node.right,
                Ordering::Equal => {
                    return true;
                },
            };
            match sub {
                Tree(None) => {
                    return false;
                },
                Tree(Some(_)) => {
                    current = sub;
                },
            }
        }
        false
    }

    fn remove(
        &mut self,
        value: &T,
    ) -> Option<()> {
        let mut current = self;

        while let Some(ref mut node) = current.0 {
            match node.value.cmp(value) {
                Ordering::Less => current = &mut current.0.as_mut().unwrap().right,
                Ordering::Greater => current = &mut current.0.as_mut().unwrap().left,
                Ordering::Equal => {
                    match (node.left.0.as_mut(), node.right.0.as_mut()) {
                        (None, None) => {
                            current.0 = None;
                            return Some(());
                        },
                        (Some(_), None) => {
                            current.0 = node.left.0.take();
                            return Some(());
                        },
                        (None, Some(_)) => {
                            current.0 = node.right.0.take();

                            return Some(());
                        },
                        (Some(_), Some(_)) => {
                            current.0.as_mut().unwrap().value = node.right.pop_min().unwrap();
                            return Some(());
                        },
                    }
                },
            }
        }
        None
    }
}
