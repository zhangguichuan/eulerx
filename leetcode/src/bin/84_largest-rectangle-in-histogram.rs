/*
给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。
求在该柱状图中，能够勾勒出来的矩形的最大面积。
示例 1:
输入：heights = [2,1,5,6,2,3]
输出：10
解释：最大的矩形为图中红色区域，面积为 10
示例 2：
输入： heights = [2,4]
输出： 4
*/

struct Solution;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut maxv = 0;
        for i in 0..n {
            let mut sk = vec![];
            let (mut l, mut r) = (i as i32, i as i32);

            //left side
            sk.push(i);
            while l >= 0 && !sk.is_empty() {
                if heights[l as usize] < heights[i as usize] {
                    sk.pop();
                    break;
                }
                l-=1;
            }
            if !sk.is_empty() {
                l = 0;
                sk.clear();
            } else {
                l+=1;
            }
            //right side
            sk.push(i);
            while r <= (n as i32 - 1) && !sk.is_empty() {
                if heights[r as usize] < heights[i as usize] {
                    sk.pop();
                    break;
                }
                r+=1;
            }
            if !sk.is_empty() {
                r = n as i32 -1;
                sk.clear();
            } else {
                r-=1;
            }
            
            //println!("{}, {}, {}, {}, {}", i, heights[i], l, r, r - l + 1);
            let v = heights[i] * (r - l + 1);
            if maxv < v {
                maxv = v;
            }
        }
        maxv as i32
    }
}

pub fn main() {
    let heights = vec![2,1,5,6,2,3];
    println!("heights is : {heights:?}"); 
    let max_v = Solution::largest_rectangle_area(heights);
    println!("max v is : {max_v}"); 

    let heights = vec![2,4];
    println!("heights is : {heights:?}"); 
    let max_v = Solution::largest_rectangle_area(heights);
    println!("max v is : {max_v}"); 
}

