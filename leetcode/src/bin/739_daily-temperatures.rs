/*
给定一个整数数组 temperatures ，表示每天的温度，返回一个数组 answer ，其中 answer[i] 是指对于第 i 天，下一个更高温度出现在几天后。如果气温在这之后都不会升高，请在该位置用 0 来代替。
示例 1:
输入: temperatures = [73,74,75,71,69,72,76,73]
输出: [1,1,4,2,1,1,0,0]
示例 2:
输入: temperatures = [30,40,50,60]
输出: [1,1,1,0]
示例 3:
输入: temperatures = [30,60,90]
输出: [1,1,0]
*/
struct Solution;
impl Solution {
    //优化版本
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut sk: Vec<usize> = vec![];
        let mut ret: Vec<i32> = vec![0; t.len()];
        for i in 0..t.len() {
            while !sk.is_empty() && t[*sk.last().unwrap()] < t[i] {
                let j = sk.pop().unwrap();
                ret[j] = (i - j) as i32;
            }
            if sk.is_empty() || t[*sk.last().unwrap()] >= t[i] {
                sk.push(i);
            }
        }
        ret
    }
    //自己写的初始版本
    pub fn daily_temperatures2(temperatures: Vec<i32>) -> Vec<i32> {
        let temps = temperatures;
        let len = temps.len();
        let mut sk: Vec<usize> = vec![];
        let mut ret: Vec<i32> = vec![0; len];
        let mut i = 0 as usize;
        while i < len {
            //println!("{:?}", sk);
            while !sk.is_empty() && temps[*sk.last().unwrap()] < temps[i] {
                let j = sk.pop().unwrap();
                ret[j] = (i - j) as i32;
            }
            sk.push(i);
            
            i+=1;
        }
        ret
    }
}

pub fn main() {
    let t: Vec<i32> = vec![73,74,75,71,69,72,76,73];
    println!("t: {:?}", t);
    let ret = Solution::daily_temperatures(t);
    println!("ret: {:?}", ret);

    let t: Vec<i32> = vec![30,60,90];
    println!("t: {:?}", t);
    let ret = Solution::daily_temperatures(t);
    println!("ret: {:?}", ret);
}
