/*
给定两个字符串 s 和 p，找到 s 中所有 p 的 异位词 的子串，返回这些子串的起始索引。不考虑答案输出的顺序。
异位词 指由相同字母重排列形成的字符串（包括相同的字符串）。
示例 1:
输入: s = "cbaebabacd", p = "abc"
输出: [0,6]
解释:
起始索引等于 0 的子串是 "cba", 它是 "abc" 的异位词。
起始索引等于 6 的子串是 "bac", 它是 "abc" 的异位词。
示例 2:
输入: s = "abab", p = "ab"
输出: [0,1,2]
解释:
起始索引等于 0 的子串是 "ab", 它是 "ab" 的异位词。
起始索引等于 1 的子串是 "ba", 它是 "ab" 的异位词。
起始索引等于 2 的子串是 "ab", 它是 "ab" 的异位词。
*/

struct Solution;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let (mut pcnt, mut ans) = (vec![0; 128], vec![]);
        for b in p.as_bytes() {
            pcnt[*b as usize] += 1;
        }
        for (i, w) in s.as_bytes().windows(p.len()).enumerate() {
            if i == 0 {
                w.iter().for_each(|b| pcnt[*b as usize] -= 1);
            } else {
                pcnt[*w.last().unwrap() as usize] -= 1;
            }
            //println!("{:?}", pcnt);
            if p.as_bytes().iter().all(|x| pcnt[*x as usize] == 0) {
                ans.push(i as i32);
            }
            //window first out
            pcnt[*w.first().unwrap() as usize] += 1;
        }

        ans
    }

    pub fn find_anagrams3(s: String, p: String) -> Vec<i32> {
        let mut window = vec![0; 128];
        let mut pcnter = vec![0; 128];

        for c in p.chars() {
            pcnter[c as usize] += 1;
        }
        let valid_total = pcnter.iter().filter(|x| **x > 0).collect::<Vec<_>>().len();
        let len = p.len();
        let mut ret = vec![];
        let s_arr = s.chars().collect::<Vec<_>>();
        let mut valid = 0;
        let mut l = 0;
        for r in 0..s_arr.len() {
            let c = s_arr[r];
            window[c as usize] += 1;
            if window[c as usize] == pcnter[c as usize] {
                valid += 1;
            }
            if r - l + 1 < len {
                continue;
            }
            while r - l + 1 >= len {
                if r - l + 1 == len && valid == valid_total {
                    ret.push(l as i32);
                }
                if window[s_arr[l] as usize] == pcnter[s_arr[l] as usize] {
                    valid -= 1;
                }

                window[s_arr[l] as usize] -= 1;
                l += 1;
            }
        }

        ret
    }

    pub fn find_anagrams2(s: String, p: String) -> Vec<i32> {
        let mut window = std::collections::HashMap::<char, i32>::new();
        let mut pcnter = std::collections::HashMap::<char, i32>::new();
        for c in p.chars() {
            pcnter.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut ret = vec![];
        let s_arr = s.chars().collect::<Vec<_>>();
        let len = p.len();
        let mut l = 0;
        let mut valid = 0;
        for r in 0..s_arr.len() {
            let c = s_arr[r];
            window.entry(c).and_modify(|v| *v += 1).or_insert(1);
            if pcnter.contains_key(&c) && *window.get(&c).unwrap() == *pcnter.get(&c).unwrap() {
                valid += 1;
            }
            if (r - l + 1) < len {
                continue;
            }
            while (r - l + 1) >= len {
                if (r - l + 1) == len && valid == pcnter.len() {
                    ret.push(l as i32);
                }

                println!("valid: {}, {}, {}", valid, l, r);
                let c = s_arr[l];
                if pcnter.contains_key(&c) && *window.get(&c).unwrap() == *pcnter.get(&c).unwrap() {
                    valid -= 1;
                }
                *window.get_mut(&c).unwrap() -= 1;
                l += 1;
            }
        }

        ret
    }
}

pub fn main() {
    let s = String::from("cbaebabacd");
    let p = String::from("abc");
    println!("s:{:?}, p:{:?}", s, p);
    let ret = Solution::find_anagrams(s, p);
    println!("{:?}", ret);

    let s = String::from("abab");
    let p = String::from("ab");
    println!("s:{:?}, p:{:?}", s, p);
    let ret = Solution::find_anagrams(s, p);
    println!("{:?}", ret);

    let s = String::from("baa");
    let p = String::from("aa");
    println!("s:{:?}, p:{:?}", s, p);
    let ret = Solution::find_anagrams(s, p);
    println!("{:?}", ret);

}

impl Solution {
    pub fn find_anagrams_x(s: String, p: String) -> Vec<i32> {
        let (mut p_cnt, mut ans) = ([0; 26], vec![]);
        for b in p.bytes() {
            p_cnt[(b - b'a') as usize] += 1;
        }
        for (i, w) in s.as_bytes().windows(p.len()).enumerate() {
            if i == 0 {
                for &b in w.iter() {
                    p_cnt[(b - b'a') as usize] -= 1;
                }
            } else {
                p_cnt[(*w.last().unwrap() - b'a') as usize] -= 1;
            }
            if p_cnt.iter().all(|&count| count == 0) {
                ans.push(i as i32);
            }
            p_cnt[(*w.first().unwrap() - b'a') as usize] += 1;
        }
        ans
    }
}
