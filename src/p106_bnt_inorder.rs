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

pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let root_node_val = postorder.last().unwrap();
    let mut left_tree_nodes= vec![];
    let mut right_tree_nodes= vec![];
    // Gather all left, and right tree tree nodes
    let mut root_hit = false;
    for i in 0..inorder.len() {
        let val = inorder[i];
        if val == *root_node_val {
            root_hit = true;
            continue;
        }
        if root_hit {
            right_tree_nodes.push(val);
        } else{
            left_tree_nodes.push(val);
        }
    }
    println!("Left: {:?}", left_tree_nodes);
    println!("Right: {:?}", right_tree_nodes);
    None
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
