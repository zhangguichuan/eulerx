/*
给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
请注意 ，必须在不复制数组的情况下原地对数组进行操作。
示例 1:
输入: nums = [0,1,0,3,12]
输出: [1,3,12,0,0]
示例 2:
输入: nums = [0]
输出: [0]
*/

struct Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut l = 0;
        for r in 0..nums.len() {
            if nums[r] != 0 {
                nums[l] = nums[r];
                if l != r {
                    nums[r] = 0; 
                }
                l+=1;
            }
        }
    }

    pub fn move_zeroes3(nums: &mut Vec<i32>) {
        let (mut p1,mut p2) = (0, 0);
        let len = nums.len();
        while p1 < len {
            if nums[p1] == 0 {
                p1+=1;
                continue ;
            }
            if p2 != p1 {
                nums[p2] = nums[p1];
            }
            p2 += 1;
            p1 += 1;
        }
        while p2 < len {
            nums[p2] = 0;
            p2+=1;
        }
    }
    pub fn move_zeroes2(nums: &mut Vec<i32>) {
        let (mut p1,mut p2) = (0, 0);
        let len = nums.len();
        while p1 < len {
            while p1 < len && nums[p1] == 0 {
                p1+=1;
            }
            if p2 != p1 && p1 < len {
                nums[p2] = nums[p1];
            }
            if p1 < len {
                p2 += 1;
            }
            
            p1 += 1;
        }
        while p2 < len {
            nums[p2] = 0;
            p2+=1;
        }
    }
}

pub fn main() {
    let mut nums = vec![0,1,0,3,12];
    println!("before: {:?}", nums);
    Solution::move_zeroes(&mut nums);    
    println!("after: {:?}", nums);

    let mut nums = vec![0];
    println!("before: {:?}", nums);
    Solution::move_zeroes(&mut nums);    
    println!("after: {:?}", nums);

    let mut nums = vec![0,1,0];
    println!("before: {:?}", nums);
    Solution::move_zeroes(&mut nums);    
    println!("after: {:?}", nums);
}


