use std::collections::VecDeque;

struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

fn main() {
    let root = build_tree();
    root.breadth_first();
    println!("");
    root.depth_first_pre();
    println!("");
    root.depth_first_post();
    println!("");
}

fn build_tree() -> TreeNode<i32> {
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

impl TreeNode<i32> {
    fn depth_first_pre(&self) {
        print!("{}, ", self.value);

        if let Some(ref left) = self.left {
            left.depth_first_pre();
        }

        if let Some(ref right) = self.right {
            right.depth_first_pre();
        }
    }

    fn depth_first_post(&self) {
        if let Some(ref left) = self.left {
            left.depth_first_post();
        }

        if let Some(ref right) = self.right {
            right.depth_first_post();
        }

        print!("{}, ", self.value);
    }

    fn breadth_first(&self) {
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while !queue.is_empty() {
            let node = queue.pop_front();

            let value = node.as_ref().unwrap().value;
            print!("{}, ", value);

            if let Some(ref left) = node.as_ref().unwrap().left {
                queue.push_back(left);
            }

            if let Some(ref right) = node.as_ref().unwrap().right {
                queue.push_back(right);
            }
        }
    }
}
