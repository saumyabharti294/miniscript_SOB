struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }

    fn height(&self) -> i32 {
        let left_height = match &self.left {
            Some(node) => node.height(),
            None => 0,
        };
        let right_height = match &self.right {
            Some(node) => node.height(),
            None => 0,
        };
        1 + left_height.max(right_height)
    }
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode::new(2))),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            })),
            right: None,
        })),
    }));

    let height = root.unwrap().height();
    println!("Height of the tree is {}", height);
}
