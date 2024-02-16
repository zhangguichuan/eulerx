/*
给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
有效字符串需满足：
1. 左括号必须用相同类型的右括号闭合。
2. 左括号必须以正确的顺序闭合。
3. 每个右括号都有一个对应的相同类型的左括号。
示例 1：
输入：s = "()"
输出：true
示例 2：
输入：s = "()[]{}"
输出：true
示例 3：
输入：s = "(]"
输出：false
 */

struct Solution;
impl Solution {
    //下面是第一种思路，另外一种思路可以讲字符串反向，再匹配一次
    pub fn is_valid(s: String) -> bool {
        let mut sk = vec![];
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                sk.push(c);
                continue ;
            }
            if c == ')' && (sk.last().is_none() || *sk.last().unwrap() != '(' ) {
                return false;
            }
            if c == ']' && (sk.last().is_none() || *sk.last().unwrap() != '[' ) {
                return false;
            }
            if c == '}' && (sk.last().is_none() || *sk.last().unwrap() != '{' ) {
                return false;
            }
            sk.pop(); 
        }
        if s.len() % 2 == 0 && sk.is_empty() {
            return true;
        }
        false
    }
}

pub fn main() {
    let s = String::from("()[]{}");
    println!("before: {s}");
    let ret = Solution::is_valid(s);
    println!("ret: {ret}");
    let s = String::from("([}}])");
    println!("before: {s}");
    let ret = Solution::is_valid(s);
    println!("ret: {ret}");
    let s = String::from("]");
    println!("before: {s}");
    let ret = Solution::is_valid(s);
    println!("ret: {ret}");
}

