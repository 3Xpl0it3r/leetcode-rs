struct Solution{}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut path = Vec::new();
        Solution::backtrack_optimize(n, k, 0,&mut path, &mut  ans);
        ans
    }

    fn backtrack(n:i32, k: i32, start_index: i32,path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>){
        if path.len() as i32 == k{
            ans.push(path.to_vec());
            return
        }
        for _i in start_index..=n{
            path.push(_i);
            Solution::backtrack(n, k, _i+1, path, ans);
            path.pop();
        }
    }

    fn backtrack_optimize(n:i32, k:i32, start_index: i32, path: &mut Vec<i32>, ans:&mut Vec<Vec<i32>>){
        if path.len() as i32 == k{
            ans.push(path.to_vec());
            return
        }
        for _i in start_index..=(n-k + path.len() as i32  + 1){
            path.push(_i);
            Solution::backtrack_optimize(n,k, _i + 1, path, ans);
            path.pop();
        }
    }
}