use std::ops::Index;

struct Solution{}

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g =g;
        let mut s = s;
        g.sort();
        s.sort();
        let mut child_index = 0;
        let mut food_index = 0;
        for(_,food) in s.iter().enumerate(){
            match g.get(child_index) {
                Some(child) => {
                    if food >= child{
                        child_index += 1;
                    }
                }
                None => {
                    return child_index as i32;
                }
            }
        }
        child_index as i32
    }
}