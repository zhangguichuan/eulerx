/*
给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是 回文串 。返回 s 所有可能的分割方案。
回文串 是正着读和反着读都一样的字符串。

示例 1：

输入：s = "aab"
输出：[["a","a","b"],["aa","b"]]
示例 2：

输入：s = "a"
输出：[["a"]]
 */
struct Solution;
impl Solution {
    //分割的思路是从起始位置，到每个进入的位置都入栈path里
    //dfs一定要有个起始位置，这算是标配了！
    pub fn partition(s: String) -> Vec<Vec<String>> {
        pub fn is_palindrome(s: &str, mut i: usize, mut j: usize) -> bool {
            if j == 0 { return true; }
            while j>=i && s[i..i+1] == s[j..j+1] {
                i += 1;
                j -= 1;
            }
            if j >= i {
                return false;
            }
            true
        }
        pub fn dfs(s: &str, k: usize, path: &mut Vec<String>, ret: &mut Vec<Vec<String>>) {
            if k == s.len() {
                ret.push(path.clone());
                return ;
            }
            for j in k..s.len() {
                if is_palindrome(s, k, j) {
                    path.push(String::from(&s[k..j+1]));
                    dfs(s, j+1, path, ret);
                    path.pop();
                }
            }
        }
        let mut ret = vec![];
        let mut path = vec![];
        dfs(&s, 0, &mut path, &mut ret);
        ret
    }

    //下面的实现方法是找出所有的回文子串，不是分割！！
    pub fn partition2(s: String) -> Vec<Vec<String>> {
        pub fn is_palindrome(s: &str, mut i: usize, mut j: usize) -> bool {
            if j == 0 { return true; }
            while j>=i && s[i..i+1] == s[j..j+1] {
                i += 1;
                j -= 1;
            }
            if j >= i {
                return false;
            }
            true
        }

        let mut ret = vec![];
        for i in 0..s.len() {
            let mut arr = vec![];
            for j in i..s.len() {
                if is_palindrome(&s, i, j) {
                    arr.push(String::from(&s[i..j+1]));     
                }
            }
            if !arr.is_empty() {
                ret.push(arr);
            }
        }
        ret 
    }
}

pub fn main() {
    let s = String::from("aac");
    println!("s: {}", s);
    let ret = Solution::partition(s);
    println!("ret: {:?}", ret);

    let s = String::from("a");
    println!("s: {}", s);
    let ret = Solution::partition(s);
    println!("ret: {:?}", ret);
}

