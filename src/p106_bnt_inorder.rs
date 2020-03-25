use std::cell::RefCell;
use std::collections::HashMap;
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

pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut map = HashMap::new();
    // Gather all left, and right tree tree nodes
    for index in 0..inorder.len() {
        let val = inorder[index];
        map.insert(val, index as i32);
    }
    // Build recursively
    build(
        &inorder,
        &postorder,
        0,
        inorder.len() as i32 - 1,
        0,
        postorder.len() as i32 - 1,
        &map,
    )
}

fn build(
    inorder: &Vec<i32>,
    postorder: &Vec<i32>,
    in_start: i32,
    in_end: i32,
    post_start: i32,
    post_end: i32,
    mapping: &HashMap<i32, i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if post_start > post_end || in_start > in_end {
        None
    } else {
        let last_value = postorder[post_end as usize];
        let node = Rc::new(RefCell::new(TreeNode::new(last_value)));

        let root_index = mapping.get(&last_value).unwrap();

        let left_child = build(
            inorder,
            postorder,
            in_start,
            root_index - 1,
            post_start,
            post_start + root_index - in_start - 1,
            mapping,
        );
        let right_child = build(
            inorder,
            postorder,
            root_index + 1,
            in_end,
            post_start + root_index - in_start,
            post_end - 1,
            mapping,
        );
        node.borrow_mut().left = left_child;
        node.borrow_mut().right = right_child;
        Some(node)
    }
}

/// Tree from example given
///         3
///        / \        
///       9  20
///         /  \
///       15    7
/// let inorder = vec![9, 3, 15, 20, 7];
/// let postorder = vec![9, 15, 7, 20, 3];
#[test]
fn calc_right_path() {
    let node1 = Rc::new(RefCell::new(TreeNode::new(3)));
    let node2 = Rc::new(RefCell::new(TreeNode::new(9)));
    let node3 = Rc::new(RefCell::new(TreeNode::new(20)));
    let node4 = Rc::new(RefCell::new(TreeNode::new(15)));
    let node5 = Rc::new(RefCell::new(TreeNode::new(7)));

    node3.borrow_mut().right = Some(node5.clone());
    node3.borrow_mut().left = Some(node4.clone());

    node1.borrow_mut().right = Some(node3.clone());
    node1.borrow_mut().left = Some(node2.clone());

    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    assert_eq!(build_tree(inorder, postorder), Some(node1));
}
