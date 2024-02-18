/*
给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。
注意：答案中不可以包含重复的三元组。
示例 1：
输入：nums = [-1,0,1,2,-1,-4]
输出：[[-1,-1,2],[-1,0,1]]
解释：
nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0 。
nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0 。
nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0 。
不同的三元组是 [-1,0,1] 和 [-1,-1,2] 。
注意，输出的顺序和三元组的顺序并不重要。
示例 2：
输入：nums = [0,1,1]
输出：[]
解释：唯一可能的三元组和不为 0 。
示例 3：
输入：nums = [0,0,0]
输出：[[0,0,0]]
解释：唯一可能的三元组和为 0 。
*/

struct Solution;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let len = nums.len();
        let mut ret = vec![];

        for i in 0..len {
            if nums[i] > 0 {
                return ret;
            }
            if i > 0 && nums[i] == nums[i-1] {
                //已经有重复的元组了,已经都找齐了,不需要再找了
                continue ;
            }
            let mut l = i + 1;
            let mut r = len - 1;
            while l < r {
                if (nums[i] + nums[l] + nums[r]) > 0 {
                    r -= 1;
                } else if (nums[i] + nums[l] + nums[r]) < 0 {
                    l += 1;
                } else {
                    ret.push(vec![nums[i], nums[l], nums[r]]); 
                    while r > l && nums[r] == nums[r-1] { r-=1; }
                    while r > l && nums[l] == nums[l+1] { l+=1; }
                    l+=1;
                    r-=1;
                }
            }
        }

        ret
    }
    pub fn three_sum2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        //组合去重用什么？hash吗？
        pub fn dfs(nums: &Vec<i32>, idx: i32, path: &mut Vec<i32>, sum: &mut i32) {
            
        }
                
        let ret = vec![];
        //dfs()
        ret
    }
}

pub fn main() {
    let nums = vec![-1,0,1,2,-1,-4];
    println!("{:?}", nums);
    let ret = Solution::three_sum(nums);
    println!("ret: {:?}", ret);

    let nums = vec![0,1,1];
    println!("{:?}", nums);
    let ret = Solution::three_sum(nums);
    println!("ret: {:?}", ret);

    let nums = vec![0,0,0];
    println!("{:?}", nums);
    let ret = Solution::three_sum(nums);
    println!("ret: {:?}", ret);

    let nums = vec![-2,0,0,2,2];
    println!("{:?}", nums);
    let ret = Solution::three_sum(nums);
    println!("ret: {:?}", ret);
}

