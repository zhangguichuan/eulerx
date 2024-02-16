/*
给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。
给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。

示例 1：
输入：digits = "23"
输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]
 */

//解析：这个两层的结构，第一层和第二层的只有组合
struct Solution;
impl Solution {
    //比letter_combinations2 的更好
    pub fn letter_combinations(digits: String) -> Vec<String> {
        pub fn dfs(digits: &str, idx: usize, path: &mut String, ans:&mut Vec<String>, c_arr: &[&str]) {
            if idx == digits.len() {
                ans.push(path.clone());
                return ;        
            }
            let d = digits[idx..idx+1].parse::<usize>().unwrap();
            let chars = c_arr[d].chars();
            for c in chars {
                path.push(c);
                dfs(digits, idx+1, path, ans, c_arr);
                path.pop();
            }
        }

        if digits.is_empty() {
            return vec![];
        }
        let c_arr = vec!["","","abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"];
        let mut ans = vec![];
        let mut path = String::from("");
        dfs(&digits, 0, &mut path, &mut ans, &c_arr);
        ans
    }
    pub fn letter_combinations2(digits: String) -> Vec<String> {
        let c_arr = vec!["","","abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"];
        pub fn dfs(digits: &str, idx: usize, path: &mut String, ans:&mut Vec<String>, c_arr: &[&str]) {
            if idx == digits.len() {
                ans.push(path.clone());
                return ;        
            }
            for i in idx..digits.len() {
                let d = digits[i..i+1].parse::<usize>().unwrap();
                let chars = c_arr[d].chars();
                for c in chars {
                    path.push(c);
                    dfs(digits, i+1, path, ans, c_arr);
                    path.pop();
                }
                if path.is_empty() {
                    return ;
                }
            }
        }

        let mut ans = vec![];
        let mut path = String::from("");
        dfs(&digits, 0, &mut path, &mut ans, &c_arr);
        ans
    }
}


pub fn main() {
    let digits = "23".to_string();
    println!("before: {:?}", digits);
    let ans = Solution::letter_combinations(digits);
    println!("ans: {:?}", ans);
    let digits = "".to_string();
    println!("before: {:?}", digits);
    let ans = Solution::letter_combinations(digits);
    println!("ans: {:?}", ans);
}
