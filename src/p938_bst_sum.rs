use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
    match root {
        Some(node) => {
            let v = node.borrow().val;
            let val = if v >= l && v <= r { v } else { 0 };
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            val + range_sum_bst2(&left, l, r) + range_sum_bst2(&right, l, r)
        }
        None => 0,
    }
}

fn range_sum_bst2(root: &Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
    match root {
        Some(node) => {
            let v = node.borrow().val;
            let val = if v >= l && v <= r { v } else { 0 };
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            val + range_sum_bst2(left, l, r) + range_sum_bst2(right, l, r)
        }
        None => 0,
    }
}

#[test]
fn calc_single_tree() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let root1 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let root2 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    assert_eq!(range_sum_bst(root, 0, 6), 5);
    assert_eq!(range_sum_bst(root1, 0, 5), 5);
    assert_eq!(range_sum_bst(root2, 0, 3), 0);
}

#[test]
fn calc_right_path() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let node1 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(3)));

    // Set right values
    let n1_rc_node = node1.clone();
    root.borrow_mut().right = Some(n1_rc_node);

    let n2_rc_node = node2.clone();
    node1.borrow_mut().right = Some(n2_rc_node);
    assert_eq!(range_sum_bst(Some(root), 2, 3), 5);
}
