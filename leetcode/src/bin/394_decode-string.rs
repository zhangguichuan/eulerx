/*
给定一个经过编码的字符串，返回它解码后的字符串。
编码规则为: k[encoded_string]，表示其中方括号内部的 encoded_string 正好重复 k 次。注意 k 保证为正整数。
你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。
此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入 
示例 1：
输入：s = "3[a]2[bc]"
输出："aaabcbc"
示例 2：
输入：s = "3[a2[c]]"
输出："accaccacc"
示例 3：
输入：s = "2[abc]3[cd]ef"
输出："abcabccdcdcdef"
示例 4：
输入：s = "abc3[cd]xyz"
输出："abccdcdcdxyz"
*/

//部分处理可以用递归处理
struct Solution;
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut digits = vec![];
        let mut nsk = vec![];
        let mut csk = vec![]; 
        let mut ret = String::from("");
       
        let mut iter_pk = s.chars().peekable();
        'end: loop {
            let c = match iter_pk.next() {
                Some(v) => v,
                None => break 'end,
            };
            if '0' <= c && c <= '9' {
                digits.push(c as i32 - '0' as i32);
                if let Some(v) = iter_pk.peek() {
                    if  *v < '0' || '9' < *v {
                        let mut sum = 0;
                        for n in digits.iter() {
                            sum = sum*10 + n;
                        }
                        nsk.push(sum); 
                        digits.clear();
                    }
                }
                continue;
            }
            if c == '[' {
                csk.push(String::from(""));
                continue ; 
            }
            if c == ']' {
                let repeat_s: String = csk.pop().unwrap();
                let mut tmp = vec![];
                let n = nsk.pop().unwrap();
                for _ in 0..n {
                    tmp.push(repeat_s.clone());
                }
                if !csk.is_empty() {
                    csk.last_mut().unwrap().push_str(&tmp.join(""));
                } else {
                    ret.push_str(&tmp.join(""));
                }
                continue ;
            }
            if !nsk.is_empty() {
                let v = csk.last_mut().unwrap();
                v.push(c);
                continue ;
            }
            ret.push(c); 
        }

        ret
    }
}

pub fn main() {
    let s = String::from("3[a]2[bc]");
    println!("{}", s);
    let ret = Solution::decode_string(s);        
    println!("ret: {}", ret);

    let s = String::from("3[a2[c]]");
    println!("{}", s);
    let ret = Solution::decode_string(s);        
    println!("ret: {}", ret);

    let s = String::from("abc3[cd]xyz");
    println!("{}", s);
    let ret = Solution::decode_string(s);        
    println!("ret: {}", ret);

    let s = String::from("100[leetcode]");
    println!("{}", s);
    let ret = Solution::decode_string(s);        
    println!("ret: {}", ret.len()/8);

    let s = String::from("10[a]");
    println!("{}", s);
    let ret = Solution::decode_string(s);        
    println!("ret: {}", ret.len());
}

/*
//用递归处理的官方解
impl Solution {
    pub fn decode_string(s: String) -> String {
        let result=Self::solve(s.as_bytes());
        unsafe{String::from_utf8_unchecked(result)}
    }

    fn solve(s:&[u8])->Vec<u8>{
        let n=s.len();
        if n==0{
            return Vec::with_capacity(0);
        }

        let mut result=Vec::with_capacity(n);
        let mut i=0;
        let mut num_buffer=Vec::new();
        while i<n{
            if s[i]>=b'a'&&s[i]<=b'z'{
                result.push(s[i]);
                i+=1;
            }else{
                while i<n&&s[i]>=b'0'&&s[i]<=b'9'{
                    num_buffer.push(s[i]);
                    i+=1;
                }
                let num:usize=String::from_utf8_lossy(&num_buffer).parse().unwrap();
                num_buffer.clear();

                let mut j=i+1;
                let mut pair_count=1;
                while j<n&&pair_count!=0{
                    if s[j]==b'['{
                        pair_count+=1;
                    }else if s[j]==b']'{
                        pair_count-=1;
                    }
                    j+=1;
                }
                let s=Self::solve(&s[i+1..j-1]);
                for _ in 0..num{
                    result.extend(s.iter().copied());
                }
                i=j;
            }
        }

        result
    }
}
 */
