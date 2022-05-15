
use std::rc::Rc;
use std::cell::RefCell;


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
            right: None
        }
    }
}

struct Solution{}


impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Solution::bfs(root)
    }

    pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> i32{

        if node.is_none(){
            return 0;
        }
        let parent = node.unwrap();
        let left_child = parent.borrow_mut().left.take();
        let right_child = parent.borrow_mut().right.take();
        if left_child.is_none() && right_child.is_none(){
            return 1;
        }
        let mut min_depth = i32::MAX;
        if left_child.is_some(){
            let left_depth = Solution::dfs(left_child);
            if left_depth <= min_depth{
                min_depth = left_depth
            }
        }
        if right_child.is_some(){
            let right_depth = Solution::dfs(right_child);
            if right_depth <= min_depth{
                min_depth = right_depth
            }
        }
        min_depth + 1

    }

    pub fn bfs(node: Option<Rc<RefCell<TreeNode>>>) -> i32{
        let mut min_depth = 0;
        if node.is_none(){
            return min_depth
        }
        let mut stack = vec![node.unwrap()];
        while !stack.is_empty(){
            min_depth += 1;
            let num = stack.len();
            for _i in 0..num{
                let top = stack.remove(0);
                let left_child = top.borrow_mut().left.take();
                let right_child = top.borrow_mut().right.take();
                if left_child.is_none() && right_child.is_none(){
                    return min_depth;
                }
                if left_child.is_some(){
                    stack.push(left_child.unwrap());
                }
                if right_child.is_some(){
                    stack.push(right_child.unwrap());
                }
            }
        }
        min_depth
    }
}