/*
给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 "" 。
注意：
对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
如果 s 中存在这样的子串，我们保证它是唯一的答案。
示例 1：

输入：s = "ADOBECODEBANC", t = "ABC"
输出："BANC"
解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。
*/

struct Solution;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let (mut scnt, mut tcnt) = ([0; 128].to_vec(), [0; 128].to_vec());
        let is_contain = |scnt: &Vec<i32>, tcnt: &Vec<i32>| -> bool {
            for i in 0..scnt.len() {
                if scnt[i] < tcnt[i] {
                    return false;
                }
            }
            return true;
        };

        for c in t.as_bytes() {
            tcnt[*c as usize] += 1;
        }

        let (mut min_w, mut left, mut right) = (s.len(), 0, 0);
        let (mut l, mut r) = (0, 0);
        //let s_arr = s.chars().collect::<Vec<_>>();
        let s_arr = s.as_bytes();

        for r in 0..s.len() {
            let r_p = s_arr[r] as usize;
            scnt[r_p] += 1;
            while is_contain(&scnt, &tcnt) {
                if r - l < min_w {
                    min_w = r - l;
                    left = l;
                    right = r + 1;
                }
                let l_p = s_arr[l] as usize;
                scnt[l_p] -= 1;
                l += 1;
            }
        }

        s[left..right].to_string()
    }

    pub fn min_window2(s: String, t: String) -> String {
        let (mut scnt, mut tcnt) = ([0; 128].to_vec(), [0; 128].to_vec());
        let is_contain = |scnt: &Vec<i32>, tcnt: &Vec<i32>| -> bool {
            for i in 0..scnt.len() {
                if scnt[i] < tcnt[i] {
                    return false;
                }
            }
            return true;
        };
        let slen = s.len();
        let tlen = t.len();

        for i in 0..tlen {
            let i = t.chars().nth(i).unwrap() as usize;
            tcnt[i] += 1;
        }

        let (mut min_w, mut left, mut right) = (slen, 0, 0);
        let (mut l, mut r) = (0, 0);

        while r < slen {
            let r_p = s.chars().nth(r).unwrap() as usize;
            scnt[r_p] += 1;
            while is_contain(&scnt, &tcnt) {
                if r - l < min_w {
                    min_w = r - l;
                    left = l;
                    right = r;
                }
                let l_p = s.chars().nth(l).unwrap() as usize;
                scnt[l_p] -= 1;
                l += 1;
            }
            r += 1;
        }

        s[left..right+1].to_string()
    }
}

pub fn main() {
    let s = "ADOBECODEBANC".to_string();
    let t = "ABC".to_string();
    let min_s = Solution::min_window(s, t);
    println!("{}", min_s);
    assert_eq!("BANC", min_s);

    let s = "a".to_string();
    let t = "aa".to_string();
    let min_s = Solution::min_window(s, t);
    println!("{}", min_s);
    assert_eq!("", min_s);
}
