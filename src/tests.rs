use crate::BinarySearchTree;

#[test]
fn bst() {
    let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();

    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
    assert_eq!(tree.min(), None);
    assert_eq!(tree.max(), None);
    assert!(!tree.contains(&5));

    assert!(tree.insert(1).is_some());

    assert!(!tree.is_empty());
    assert_eq!(tree.len(), 1);
    assert_eq!(tree.min(), Some(&1));
    assert_eq!(tree.max(), Some(&1));
    assert!(!tree.contains(&2));
    assert!(tree.contains(&1));

    assert!(tree.insert(0).is_some());
    assert!(tree.insert(-1).is_some());
    assert!(tree.insert(2).is_some());
    assert!(tree.insert(2).is_none());
    assert!(tree.insert(4).is_some());
    assert!(tree.insert(-10).is_some());
    assert!(tree.insert(3).is_some());
    assert!(tree.insert(0).is_none());

    assert_eq!(tree.size, 7);

    assert_eq!(tree.min().unwrap(), &-10);
    assert_eq!(tree.max().unwrap(), &4);

    assert!(tree.contains(&-10));
    assert!(tree.contains(&0));
    assert!(!tree.contains(&123));

    assert!(tree.remove(&100).is_none());
    assert!(tree.remove(&-1).is_some());
    assert!(tree.remove(&0).is_some());
    assert!(tree.remove(&2).is_some());
    assert!(tree.remove(&4).is_some());
    assert!(tree.remove(&1).is_some());
    assert!(tree.remove(&-10).is_some());
    assert!(tree.remove(&5).is_none());
    assert!(tree.remove(&0).is_none());
    assert!(tree.remove(&3).is_some());

    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
    assert_eq!(tree.min(), None);
    assert_eq!(tree.max(), None);
}