struct Solution{}
impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        if nums.is_empty(){
            return 0
        }
        let mut nums = nums;
        nums.sort();
        let mut count  = 0;
        let middle = nums[nums.len()/2];
        for i in 0..nums.len(){
            count += i32::abs(nums[i] - middle);
        }
        count
    }
}