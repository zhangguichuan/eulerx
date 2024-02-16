/*
给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。
示例 1：

输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
示例 2：

输入：nums = [0,1]
输出：[[0,1],[1,0]]
*/

struct Solution;
impl Solution {

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        pub fn dfs(nums: &Vec<i32>, ans: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
            if path.len() == nums.len() {
                ans.push(path.clone());
                return ;
            }
            for i in 0..nums.len() {
                if path.contains(&nums[i]) {
                    continue ;
                }
                path.push(nums[i]);
                dfs(&nums, ans, path);
                path.pop();
            }
        }
        
        let mut path = vec![];
        let mut ans = vec![];
        dfs(&nums, &mut ans, &mut path);
        ans
    }
}

pub fn main() {
    let arr = vec![1,2,3];
    println!("arr: {:?}", arr);
    let ans = Solution::permute(arr);
    println!("ans: {:?}", ans);
}
