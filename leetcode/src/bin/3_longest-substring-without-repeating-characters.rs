/*
给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
示例 1:
输入: s = "abcabcbb"
输出: 3
解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
示例 2:
输入: s = "bbbbb"
输出: 1
解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
示例 3:
输入: s = "pwwkew"
输出: 3
解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
*/

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut record = vec![0; 128];
        let mut ret = 0;
        let mut l = 0;
        s.chars().enumerate().for_each(|(i, c)| {
            l = l.max(record[c as usize]); 
            ret = ret.max(i - l + 1);
            record[c as usize] = i + 1;
        });
        ret as i32
    }
    
    pub fn length_of_longest_substring2(s: String) -> i32 {
        let mut cnter = std::collections::HashMap::<char, i32>::new();
        let mut ret = 0;
        let chars = s.chars().collect::<Vec<_>>();
        let mut l = 0;
        for r in 0..s.len() {
            let c = chars[r];
            if !cnter.contains_key(&c) {
                cnter.insert(c, 1);
            } else {
                let ref_cnt = cnter.get_mut(&c).unwrap();
                *ref_cnt += 1;
            }
            while *cnter.get(&c).unwrap() == 2 {
                *cnter.get_mut(&chars[l]).unwrap() -= 1;
                l+=1;
            }
            ret = ret.max(r - l + 1);
        }
        
        ret as i32
    }
}

pub fn main() {
    let s = String::from("abcabcbb");
    println!("{:?}", s);
    let ret = Solution::length_of_longest_substring(s);
    println!("{:?}", ret);

    let s = String::from("bbbbb");
    println!("{:?}", s);
    let ret = Solution::length_of_longest_substring(s);
    println!("{:?}", ret);

    let s = String::from("a");
    println!("{:?}", s);
    let ret = Solution::length_of_longest_substring(s);
    println!("{:?}", ret);
}
