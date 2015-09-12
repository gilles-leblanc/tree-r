use std::collections::VecDeque;

struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn main() {
    let root = build_tree();
    root.breadth_first();
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

    fn depth_first_post(self) {
        if self.left.is_some() {
            self.left.unwrap().depth_first_post();
        }

        if self.right.is_some() {
            self.right.unwrap().depth_first_post();
        }

        print!("{}, ", self.value);
    }

    fn breadth_first(self) {
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while !queue.is_empty() {
            let node = queue.pop_front();

            match node {
                Some(e) => {
                    print!("{}, ", e.value);

                    if e.left.is_some() {
                        queue.push_back(*e.left.unwrap());
                    }

                    if e.right.is_some() {
                        queue.push_back(*e.right.unwrap());
                    }
                },
                None => return,
            }
        }
    }
}
