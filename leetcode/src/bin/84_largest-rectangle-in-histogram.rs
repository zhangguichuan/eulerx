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
        let mut tmp = vec![];
        tmp.push(-1);
        tmp.extend(heights);
        tmp.push(-1);
        let mut sk = vec![];
        let mut ret = 0;

        for i in 0..tmp.len() {
            //while let Some(&top) = sk.last() {
                //if tmp[top] <= tmp[i] { 
                //    break;
                //}
            while !sk.is_empty() && tmp[*sk.last().unwrap()] > tmp[i] {
                let top = sk.pop().unwrap();
                let h = tmp[top];
                let w = if sk.is_empty() { i } else { i - *sk.last().unwrap() - 1 };
                ret = std::cmp::max(ret, h*w as i32);
            }
            sk.push(i);
        }
        ret
    }
    pub fn largest_rectangle_area3(mut heights: Vec<i32>) -> i32 {
        let mut min_l = 0;
        let mut sk = vec![];
        let mut r = 0;
        heights.push(-1); 
        let len = heights.len();
        let mut ret = 0 as i32;

        while r < len {
            sk.push(r);
            let mut _r = r + 1;
            while !sk.is_empty() && _r < len {
                //println!("{:?}", sk);
                while !sk.is_empty() && heights[_r] >= heights[*sk.last().unwrap()] {
                    sk.push(_r);
                    _r += 1;
                }
                while !sk.is_empty() && heights[_r] < heights[*sk.last().unwrap()] {
                    let j = sk.pop().unwrap();
                    if sk.is_empty() {
                        break;    
                    } else {
                        let l = *sk.last().unwrap();
                        let area = (_r - l - 1) as i32 * heights[j];
                        ret = ret.max(area as i32);
                    }
                }
                if !sk.is_empty() {
                    sk.push(_r);
                    _r += 1;
                }
            }
            //left
            let area = (_r - min_l) as i32 * heights[r];
            println!("r:{},_r:{},min:{},area:{}", r, _r, min_l,area);
            ret = ret.max(area as i32);
             
            r = _r;
        }

        ret
    }
    //这个两头找的版本会超出时间限制!!
    pub fn largest_rectangle_area2(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut maxv = 0;
        let mut sk = vec![];
        for i in 0..n {
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

    let heights = vec![8180,8180,8180,8180,8180,8180];
    println!("heights is : {:?}", heights); 
    let max_v = Solution::largest_rectangle_area(heights);
    println!("max v is : {max_v}"); 

    let heights = vec![0];
    println!("heights is : {heights:?}"); 
    let max_v = Solution::largest_rectangle_area(heights);
    println!("max v is : {max_v}"); 

    let heights = vec![4,2,0,3,2,5];
    println!("heights is : {heights:?}"); 
    let max_v = Solution::largest_rectangle_area(heights);
    println!("max v is : {max_v}"); 

    let heights = vec![4,2,0,2,2,1,1,2];
    println!("heights is : {heights:?}"); 
    let max_v = Solution::largest_rectangle_area(heights);
    println!("max v is : {max_v}"); 

}

/*
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let len = heights.len();
        let mut tmp = vec![0];  // 创建一个包含一个0的向量
        tmp.extend_from_slice(&heights);  // 将heights中的元素拷贝到tmp中
        tmp.push(0);  // 在tmp的末尾添加一个0

        let mut stack = Vec::new();
        let mut res = 0;
        for i in 0..tmp.len() {
            while let Some(&top) = stack.last() {
                if tmp[top] <= tmp[i] { break; }
                stack.pop();
                let height = tmp[top];
                let width = if stack.is_empty() { i } else { i - stack.last().unwrap() - 1 };
                res = std::cmp::max(res, height as usize * width);
            }
            stack.push(i);
        }
        res as i32
    }
}
*/
