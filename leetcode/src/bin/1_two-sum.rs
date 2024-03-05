/*
给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
你可以按任意顺序返回答案。
示例 1：
输入：nums = [2,7,11,15], target = 9
输出：[0,1]
解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。

示例 2：
输入：nums = [3,2,4], target = 6
输出：[1,2]

示例 3：
输入：nums = [3,3], target = 6
输出：[0,1]
*/

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::<i32, usize>::new();
        let mut ans = vec![];
        for (i, v) in nums.into_iter().enumerate() {
            if map.contains_key(&(target - v)) {
                ans.push(*map.get(&(target-v)).unwrap() as i32);
                ans.push(i as i32);
                return ans;
            }
            map.insert(v, i);
        }
        ans
    }


    //这个方法是先遍历放入了map,更好的是一边遍历，一边放
    pub fn two_sum3(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::<i32, usize>::new();
        for (i,v) in nums.iter().enumerate() {
            map.insert(*v, i);
        }
        
        for (i,v) in nums.iter().enumerate() {
            let l = target - *v;
            if map.contains_key(&l) {
                return vec![i as i32, *map.get(&l).unwrap() as i32];
            }
        }
        vec![]
    }

    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![];
        for (i, v) in nums.iter().enumerate() {
            let l = target - *v;
            for (j, v) in nums[i+1..].iter().enumerate() {
                if l == *v {
                    ans.push(i as i32);
                    ans.push((i + 1 + j) as i32);
                    return ans;
                }
            }
        }
        vec![]
    }
}

pub fn main() {
    let nums = vec![1,2,3,4];
    let target = 4;
    println!("{:?}, {}", nums, target);
    let ans = Solution::two_sum(nums, target);
    println!("ans: {:?}", ans);

    let nums = vec![3,3];
    let target = 6;
    println!("{:?}, {}", nums, target);
    let ans = Solution::two_sum(nums, target);
    println!("ans: {:?}", ans);
}

