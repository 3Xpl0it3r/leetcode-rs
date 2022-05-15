struct Solution{}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut path = Vec::new();
        Solution::backtrack(k,n, 1, 0,&mut path, &mut ans);
        ans
    }

    pub fn backtrack(k:i32, n: i32,start_index: i32,mut sum: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>){
        if sum > n{
            return
        }
        if path.len() as i32 == k{
            if sum == n{
                ans.push(path.to_vec());
            }
            return
        }
        for _i in start_index..=9{
            path.push(_i);
            sum += _i;
            Solution::backtrack(k, n, _i + 1, sum+_i as i32 , path, ans);
            path.pop();
            sum -= _i as i32 ;
        }
    }
}