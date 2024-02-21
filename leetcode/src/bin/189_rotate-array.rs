/*
给定一个整数数组 nums，将数组中的元素向右轮转 k 个位置，其中 k 是非负数。
示例 1:
输入: nums = [1,2,3,4,5,6,7], k = 3
输出: [5,6,7,1,2,3,4]
解释:
向右轮转 1 步: [7,1,2,3,4,5,6]
向右轮转 2 步: [6,7,1,2,3,4,5]
向右轮转 3 步: [5,6,7,1,2,3,4]
示例 2:
输入：nums = [-1,-100,3,99], k = 2
输出：[3,99,-1,-100]
解释: 
向右轮转 1 步: [99,-1,-100,3]
向右轮转 2 步: [3,99,-1,-100]
*/

struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        pub fn reverse(arr: &mut Vec<i32>, start: usize, end: usize) {
            //for i in 0..(end - start)/2 {
            //    arr.swap(start + i, end - i - 1); 
            //}
            let (mut l,mut r) = (start,  end - 1);
            let mut tmp = 0;
            while l < r {
                tmp = arr[l];
                arr[l] = arr[r];
                arr[r] = tmp; 
                l+=1;
                r-=1;
            }
        
        }
        
        let k = k as usize % nums.len();
        reverse(nums, 0, nums.len());
        reverse(nums, 0, k as usize);
        reverse(nums, k as usize, nums.len());
    }
}

pub fn main() {
    let mut nums = vec![1,2,3,4,5,6,7]; 
    let k = 3;
    println!("nums: {:?}, k: {}", nums, k);
    Solution::rotate(&mut nums, k); 
    println!("ans: {:?}", nums);

    let mut nums = vec![1,2,3]; 
    let k = 4;
    println!("nums: {:?}, k: {}", nums, k);
    Solution::rotate(&mut nums, k); 
    println!("ans: {:?}", nums);
}
