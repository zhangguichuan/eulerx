/*
给你一个整数数组 nums 和一个整数 k, 请你统计并返回 该数组中和为 k 的子数组的个数.
子数组是数组中元素的连续非空序列.
示例 1：
输入：nums = [1,1,1], k = 2
输出：2
示例 2：
输入：nums = [1,2,3], k = 3
输出：2
*/

struct Solution;
impl Solution {

    //前缀和, 先保存 前缀和 和 哈希表
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut presum = vec![0; nums.len() + 1];
        for i in 1..=nums.len() {
            presum[i] = presum[i-1] + nums[i-1];
        }
        println!("{:?}", presum);
        let mut map = std::collections::HashMap::<i32, usize>::new();
        map.insert(0,1);
        //let mut hmap = HashMap::from([(0, 1)]);
        for i in 1..=nums.len() {
            if map.contains_key(&(presum[i] - k)) {
                ans += *map.get(&(presum[i] - k)).unwrap();
            }
            map.entry(presum[i]).and_modify(|x| *x+=1).or_insert(1); 
            println!("{:?}", map);
        }
        ans as i32
    }

    //暴力枚举,移动位置是左边, O(n^2)
    pub fn subarray_sum2(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in (0..=i).rev() {
                sum += nums[j];
                if sum == k {
                    ans += 1;
                }
            }
        }
        ans
    }

    //有负数的场景，无法用双指针，用暴力枚举法或者前缀和比较好
    pub fn subarray_sum3(nums: Vec<i32>, k: i32) -> i32 {
        let (mut l, mut ret) = (0, 0); 
        let mut sum = 0;
        for r in 0..nums.len() {
            sum += nums[r];
            if sum < k {
                continue ;
            }
            while l <= r && sum != k {
                if sum == k {
                    ret += 1;
                }
                sum -= nums[l]; 
                l+=1;
            }
        }

        ret
    }
}


pub fn main() {
    let nums = vec![1,1,1];
    let k = 2;
    println!("nums: {:?}, k: {}", nums, k);
    let ret = Solution::subarray_sum(nums, k);
    println!("{:?}", ret);

    let nums = vec![-1,-1,1];
    let k = 0;
    println!("nums: {:?}, k: {}", nums, k);
    let ret = Solution::subarray_sum(nums, k);
    println!("{:?}", ret);

    let nums = vec![1];
    let k = 0;
    println!("nums: {:?}, k: {}", nums, k);
    let ret = Solution::subarray_sum(nums, k);
    println!("{:?}", ret);

    let nums = vec![1,-1,0];
    let k = 0;
    println!("nums: {:?}, k: {}", nums, k);
    let ret = Solution::subarray_sum(nums, k);
    println!("{:?}", ret);
}


use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum_x(nums: Vec<i32>, k: i32) -> i32 {
        let mut hmap = HashMap::from([(0, 1)]);
        let mut pre = 0;
        let mut count = 0;
        for i in nums.into_iter() {
            pre += i;
            hmap.entry(pre - k).and_modify(|n| count += *n);
            hmap.entry(pre).and_modify(|n| *n += 1).or_insert(1);
        }
        count
    }
}


