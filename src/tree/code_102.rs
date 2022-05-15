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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none(){
            return vec![];
        }
        let mut ans = Vec::new();
        let mut stack = vec![root.unwrap()];
        while !stack.is_empty(){
            let mut level_elm = Vec::new();
            let num = stack.len();
            for _i in 0..num {
                let mut top = stack.remove(0);
                level_elm.push(top.borrow_mut().val);
                if top.borrow_mut().left.is_some(){
                    stack.push(top.borrow_mut().left.take().unwrap());
                }
                if top.borrow_mut().right.is_some(){
                    stack.push(top.borrow_mut().right.take().unwrap());
                }
            }
            ans.push(level_elm);
        }
        ans
    }
}