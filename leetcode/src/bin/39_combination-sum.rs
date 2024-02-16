/*
给你一个 无重复元素 的整数数组 candidates 和一个目标整数 target ，找出 candidates 中可以使数字和为目标数 target 的 所有 不同组合 ，并以列表形式返回。你可以按 任意顺序 返回这些组合。
candidates 中的 同一个 数字可以 无限制重复被选取 。如果至少一个数字的被选数量不同，则两种组合是不同的。
对于给定的输入，保证和为 target 的不同组合数少于 150 个。

输入：candidates = [2,3,6,7], target = 7
输出：[[2,2,3],[7]]
解释：
2 和 3 可以形成一组候选，2 + 2 + 3 = 7 。注意 2 可以使用多次。
7 也是一个候选， 7 = 7 。
仅有这两种组合。

*/

//解析:1. 递归的层要遍历的数是可以全选的，2.递归的层数是满足target的目标为终止
struct Solution;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        pub fn dfs(candts: &Vec<i32>, idx: usize, target: i32, sum: i32, path: &mut Vec<i32>, ans:&mut Vec<Vec<i32>>) {
            if sum > target {
                return ;
            }
            if target == sum {
                ans.push(path.clone());
                return ;
            }
            for i in idx..candts.len() {
                let n = candts[i]; 
                path.push(n);
                dfs(candts, i, target, sum+n, path, ans);
                path.pop();
            }
        }

        let mut ans = vec![];
        let mut path = vec![];
        dfs(&candidates, 0, target, 0, &mut path, &mut ans); 
        ans
    }
}

pub fn main() {
    let candts = vec![2,3,6,7];
    let target = 7;
    println!("before: {:?}", candts);
    let ans = Solution::combination_sum(candts, target);
    println!("before: {:?}", ans);
    let candts = vec![2];
    let target = 1;
    println!("before: {:?}", candts);
    let ans = Solution::combination_sum(candts, target);
    println!("before: {:?}", ans);
}
