// 53 最大子数和
struct Solution{}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty(){
            return 0;
        }
        let mut max_sum = nums[0];
        let mut current_sum = 0;
        for (_, v) in nums.iter().enumerate(){
            current_sum += v;
            if current_sum > max_sum{
                max_sum = current_sum
            }
            if current_sum < 0{
                current_sum = 0
            }
        }
        max_sum
    }
}

#[cfg(test)]
mod test{
    use super::Solution;
    #[test]
    fn test(){
        let case1 = vec![-2,1,-3,4,-1,2,1,-5,4];
        let case2 = vec![1];
        let case3 = vec![5,4,-1,7,8];
        assert_eq!(6, Solution::max_sub_array(case1));
        assert_eq!(1, Solution::max_sub_array(case2));
        assert_eq!(23, Solution::max_sub_array(case3));
    }
}