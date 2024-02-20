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
    //只用一个变量的简单写法:由于求最值只用到了最近一个最优值，因此只用一个变量即可!
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        //let mut dp = vec![0; nums.len()];
        let mut pre = nums[0];
        let mut ans = pre;
        for i in 1..nums.len() {
            pre = std::cmp::max(pre + nums[i], nums[i]);
            if ans < pre {
                ans = pre;
            }
            //let v = std::cmp::max(pre + nums[i], nums[i]);
            //if ans < pre {
            //   ans = pre;
            //}
            //pre = v;
        }
        ans
    }
    //以游标为右边界，最大值包含的数组都是包含右边界的，并且都是连续的，游标向右移动过程中，存储下包含游标的最大值，就是状态转移数组。关键是游标向右移动过程中都是被包含的，所以都是连续数组
    //要思考游标是从前向后的，每次的包含游标的连续数组都是一种组合，并且能够保证不重复，这和前缀和的暴力枚举思路是相似的，也和此题的暴力枚举的思路是相似的，重点是包含游标的连续数组,能够不重合
    //这也是遍历数组的每一种连续组合的过程，需要遍历数组的每一种连续的组合，并且求最值的场景，都可以使用这种方法！
    //状态转移方程：dp[i] = max(dp[i-1] + nums[i], nums[i])
    pub fn max_sub_array3(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        let mut ans = dp[0];
        for i in 1..nums.len() {
            dp[i] = std::cmp::max(dp[i-1] + nums[i], nums[i]);
            if ans < dp[i] {
                ans = dp[i];
            }
        }
        ans
    }
    //暴力枚举
    pub fn max_sub_array2(nums: Vec<i32>) -> i32 {
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

    let nums = vec![5,4,-1,7,8];
    println!("{:?}", nums);
    let ret = Solution::max_sub_array(nums);
    println!("{:?}", ret);
}

