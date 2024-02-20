/*
给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和.
子数组 是数组中的一个连续部分。
示例 1：
输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
输出：6
解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
示例 2：
输入：nums = [1]
输出：1
示例 3：
输入：nums = [5,4,-1,7,8]
输出：23
*/

struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
     
    }
    //暴力枚举
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ret = i32::MIN;
        for l in 0..nums.len() {
            let mut sum = 0;
            for i in (0..=l).rev() {
                sum += nums[i];
                if sum > ret {
                    ret = sum;
                }
            }
        }
        ret
    }
}

pub fn main() {
    let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
    println!("{:?}", nums);
    let ret = Solution::max_sub_array(nums);
    println!("{:?}", ret);

    let nums = vec![-1];
    println!("{:?}", nums);
    let ret = Solution::max_sub_array(nums);
    println!("{:?}", ret);
}

