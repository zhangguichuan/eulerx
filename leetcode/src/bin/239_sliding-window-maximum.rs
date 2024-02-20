/*
给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。
返回 滑动窗口中的最大值 。
示例 1：
输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
输出：[3,3,5,5,6,7]
解释：
滑动窗口的位置                最大值
---------------               -----
[1  3  -1] -3  5  3  6  7       3
 1 [3  -1  -3] 5  3  6  7       3
 1  3 [-1  -3  5] 3  6  7       5
 1  3  -1 [-3  5  3] 6  7       5
 1  3  -1  -3 [5  3  6] 7       6
 1  3  -1  -3  5 [3  6  7]      7
示例 2：
输入：nums = [1], k = 1
输出：[1]
*/
struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {

        vec![]
    }

    pub fn max_sliding_window2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut sk = vec![]; 
        let mut l = 0;
        let mut ans = vec![];
        for r in 0..nums.len() {
            while !sk.is_empty() && nums[r] > *sk.last().unwrap() {
                sk.pop();
            }
            sk.push(nums[r]); 

            if r - l + 1 < k {
                continue ;
            }

            while r - l + 1 >= k {
                if r -l + 1 == k {
                    ans.push(*sk.first().unwrap());
                }
                if nums[l] == *sk.first().unwrap() {
                    sk.remove(0);
                }
                l+=1;
            }
        }
        ans
    }
}

pub fn main() {
    let nums = vec![1,3,-1,-3,5,3,6,7]; 
    let k = 3;
    println!("nums: {:?}, k: {}", nums, k);
    let ret = Solution::max_sliding_window(nums, k);
    println!("{:?}", ret);

    let nums = vec![1,3,-1,-3,2,5,3,6,7]; 
    let k = 3;
    println!("nums: {:?}, k: {}", nums, k);
    let ret = Solution::max_sliding_window(nums, k);
    println!("{:?}", ret);
}
