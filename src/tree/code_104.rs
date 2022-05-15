
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none(){
            return 0;
        }
        let mut max_depth: i32 = 0;
        let mut stack = vec![root.unwrap()];
        while !stack.is_empty() {
            let num = stack.len();
            for _i in 0..num{
                let top = stack.remove(0);
                if top.borrow_mut().left.is_some(){
                    stack.push(top.borrow_mut().left.take().unwrap());
                }
                if top.borrow_mut().right.is_some(){
                    stack.push(top.borrow_mut().right.take().unwrap());
                }
            }
            max_depth+=1;
        }
        max_depth
    }
}