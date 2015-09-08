
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn main() {
    let root = build_tree();
    root.depth_first_pre();
}

fn build_tree() -> TreeNode {
    let root = TreeNode { value: 2,
        left: Some(Box::new(TreeNode { value: 7,
                            left: Some(Box::new(TreeNode { value: 2, left: None, right: None })),
                            right: Some(Box::new(TreeNode { value: 6,
                                                left: Some(Box::new(TreeNode { value: 5, left: None, right: None })),
                                                right: Some(Box::new(TreeNode { value: 11, left: None, right: None })) })) })),
        right: Some(Box::new(TreeNode { value: 5,
                            left: None,
                            right: Some(Box::new(TreeNode { value: 9,
                                                left: Some(Box::new(TreeNode { value: 4, left: None, right: None })),
                                                right: None })) }))};
    return root;
}

impl TreeNode {
    fn depth_first_pre(self) {
        print!("{}, ", self.value);

        if self.left.is_some() {
            self.left.unwrap().depth_first_pre();
        }

        if self.right.is_some() {
            self.right.unwrap().depth_first_pre();
        }
    }
}
