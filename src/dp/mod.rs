use std::collections::HashMap;
use std::borrow::BorrowMut;

mod code_509_fib;

struct Solution{}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        return Solution::iterator(n);
    }

    fn iterator(n: i32) -> i32{
        if n <= 1 {
            return n;
        }
        let mut dp_table =vec![0;(n+1) as usize];
        dp_table[0] = 0;
        dp_table[1] = 1;
        for _i in 2..=n{
            dp_table[_i as usize] = dp_table[(_i-1)as usize] + dp_table[(_i-2) as usize];
        }
        dp_table[n as usize]
    }

}