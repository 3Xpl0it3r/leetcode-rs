struct Solution{}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.is_empty(){
            return 0;
        }
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut cur_area = 0;
        while left < right{
            if height[left] < height[right]{
                cur_area = height[left] as i32 * (right - left) as i32;
                left += 1;
            }else {
                cur_area = height[right] as i32  * (right - left) as i32;
                right -= 1;
            }
            if cur_area > max_area{
                max_area = cur_area;
            }
        }
        max_area
    }
}

#[cfg(test)]
mod test{
    use super::Solution;
    #[test]
    fn test(){
        let input = vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(49, Solution::max_area(input));
    }

}