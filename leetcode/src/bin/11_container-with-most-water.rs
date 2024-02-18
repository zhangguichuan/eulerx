/*
给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i])
找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
返回容器可以储存的最大水量。
说明：你不能倾斜容器。
示例 1：
输入：[1,8,6,2,5,4,8,3,7]
输出：49
解释：图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
示例 2：
输入：height = [1,1]
输出：1
*/

struct Solution; 
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len()-1);
        let mut ret = 0_i32;
        while l < r {
            ret = ret.max((r - l) as i32 * std::cmp::min(height[l], height[r]));
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        ret
    }
}

pub fn main() {
    let height = vec![1,8,6,2,5,4,8,3,7];
    println!("{:?}", height);
    let ret = Solution::max_area(height);
    println!("ret: {:?}", ret);

    let height = vec![1,1];
    println!("{:?}", height);
    let ret = Solution::max_area(height);
    println!("ret: {:?}", ret);
}

