/*
数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。

示例 1：
输入：n = 3
输出：["((()))","(()())","(())()","()(())","()()()"]
示例 2：
输入：n = 1
输出：["()"] 
*/

//解析：每一层的递归，可以选择递归产生()，也可以选择不递归产生()
//全排列：如何进行减枝?
struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        pub fn dfs(n: i32, cnt: i32, path: &mut String, ans: &mut Vec<String>) {
            if path.len() > (2*n as usize) {
                return ;
            }
            if path.len() == (2*n as usize) && cnt == 0 {
                ans.push(path.clone());
                return ;
            }
            for k in vec!['(',')'] {
                if k == '(' {
                    path.push(k);
                    dfs(n, cnt+1, path, ans);
                    path.pop();
                }

                if k == ')' && cnt > 0 {
                    path.push(k);
                    dfs(n, cnt-1, path, ans);
                    path.pop();
                }
            }
        }
        let mut ans = vec![];
        let mut path = String::from("");
        dfs(n, 0, &mut path, &mut ans);
        ans
    }
}

pub fn main() {
    let n = 3;
    println!("n: {}", n);
    let ans = Solution::generate_parenthesis(n);
    println!("ans: {:?}", ans);
    let n = 1;
    println!("n: {}", n);
    let ans = Solution::generate_parenthesis(n);
    println!("ans: {:?}", ans);
}
