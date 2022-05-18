
struct Solution{}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut count  = 0;
        let mut prev_diff = 0;
        let mut cur_diff = 0;
        for i in 0..nums.len()-1 {
            cur_diff = nums[i+1] - nums[i];
            if (cur_diff > 0 && prev_diff <= 0) || (cur_diff < 0 && prev_diff >=0){
                prev_diff = cur_diff;
                count += 1;
            }
        }
        count
    }
}