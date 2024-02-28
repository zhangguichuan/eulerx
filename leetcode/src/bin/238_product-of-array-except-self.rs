/*
给你一个整数数组 nums，返回 数组 answer ，其中 answer[i] 等于 nums 中除 nums[i] 之外其余各元素的乘积 。
题目数据 保证 数组 nums之中任意元素的全部前缀元素和后缀的乘积都在  32 位 整数范围内。
请 不要使用除法，且在 O(n) 时间复杂度内完成此题。
示例 1:
输入: nums = [1,2,3,4]
输出: [24,12,8,6]
示例 2:
输入: nums = [-1,1,0,-3,3]
输出: [0,0,9,0,0]
*/

struct Solution;
impl Solution {
    //和前缀和类似:使用前缀积和后缀积
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut predot = vec![1; nums.len()+1];
        let mut postdot = vec![1; nums.len()+1];
        let mut ans = vec![];
        for i in 1..=nums.len() {
            predot[i] = predot[i-1] * nums[i-1];
        }
        for i in (0..nums.len()).rev() {
            postdot[i] = postdot[i+1] * nums[i];
        }
        for i in 0..nums.len() {
            ans.push(predot[i] * postdot[i+1])
        }
        
        ans
    }
}

pub fn main() {
    let nums = vec![1,2,3,4];    
    println!("{:?}", nums);
    let ans = Solution::product_except_self(nums);
    println!("ans: {:?}", ans);

    let nums = vec![-1,1,0,-3,3];
    println!("{:?}", nums);
    let ans = Solution::product_except_self(nums);
    println!("ans: {:?}", ans);
}


impl Solution {
  //官网解
  pub fn product_except_self_x(nums: Vec<i32>) -> Vec<i32> {
    let length = nums.len();
    let mut answer = vec![0; length];
    let mut left = vec![1; length];
    let mut right = vec![1; length];

    for i in 1..length {
      left[i] = nums[i - 1] * left[i - 1];
      right[length - i - 1] = nums[length - i] * right[length - i];
    }


    for i in 0..length {
      answer[i] = left[i] * right[i];
    }

    return answer;
  }
}