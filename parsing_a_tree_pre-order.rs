use std::rc::Rc;
use std::cell::RefCell;

// Define the tree node structure
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

// Parse the tree from pre-order traversal string
fn build_tree(preorder: &mut Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() {
        return None;
    }
    
    let root_val = preorder.remove(0);
    let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
    
    if !preorder.is_empty() && preorder[0] < root_val {
        root.borrow_mut().left = build_tree(preorder);
    }
    
    if !preorder.is_empty() && preorder[0] > root_val {
        root.borrow_mut().right = build_tree(preorder);
    }
    
    Some(root)
}

// Test the tree parsing function
fn main() {
    let mut preorder = vec![8, 5, 1, 7, 10, 12];
    let root = build_tree(&mut preorder).unwrap();
    println!("{:#?}", root);
}
