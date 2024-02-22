/*
给你一个未排序的整数数组 nums ，请你找出其中没有出现的最小的正整数。
请你实现时间复杂度为 O(n) 并且只使用常数级别额外空间的解决方案。
示例 1：
输入：nums = [1,2,0]
输出：3
示例 2：
输入：nums = [3,4,-1,1]
输出：2
示例 3：
输入：nums = [7,8,9,11,12]
输出：1
*/

//思路：使用正数与数组下标的对应关系：n-1 = i,  n>=1的正数，i 是数组下标从0开始的正数。
//将nums[i] 看做是二级指针:数组下标, 这样将nums[nums[i]-1]
//的数值标位负数，这样就表示在[1,n]区间的是在数组中都是负数！最后找到数组中第一个正数的下标，返回下标+1即可！
struct Solution;
impl Solution {
    //利用正数和数组下标的对应关系
    //优化下面的代码，标记改为相反的负数，并且使用绝对值来获取地址，这是技巧
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len  = nums.len();
        for i in 0..len {
            if nums[i] <= 0 {
                nums[i] = len as i32 + 1;
            }
        }
        for i in 0..len {
            //再用abs()取值，不影响原来的数值
            let mut p_int = (nums[i].abs() - 1) as usize;
            /*
            if 0 <= p_int && p_int < len {
                //使用 -nums[p_int] 相当于保存了原值, 注意这里用 -nums[p_int].abs(),必须是绝对值
                //不用abs()可能会被指向两次，这样可能就变回原来的数值了！比如:[1,1]
                nums[p_int] = -nums[p_int].abs()
            }
            */
            //或者判断nums[p_int] 是否是大于0，大于0说明是
            if p_int < len && nums[p_int] > 0 {
                nums[p_int] = -nums[p_int]
            }
        }
        for i in 0..len {
            if nums[i] > 0 {
                return (i+1) as i32;
            }
        }
        return (len + 1) as i32;
    }
    //利用正数和数组下标的对应关系
    pub fn first_missing_positive2(mut nums: Vec<i32>) -> i32 {
        let len  = nums.len();
        for i in 0..len {
            if nums[i] <= 0 {
                nums[i] = len as i32 + 1;
            }
        }
        /*
        //这里rust阻止了：迭代器中修改迭代器中的值！因为这样会导致迭代发生错误！
        for n in nums.iter_mut() {
            if *n <= len as i32 {
                let idx = *n as usize - 1;
                nums[idx] = -1;
            }
        }
        */
        /*
        //这里有问题，因为迭代过程中修改了迭代的值，导致迭代过程中取索引就发生了错误
        //如果使用rust的迭代器，是不会发生这样的问题的！因为触犯了rust的借用规则!
        //idx 相当于二级指针了！
        for i in 0..len {
            println!("a:{:?}", nums);
            if nums[i] <= len as i32 {
                let idx = (nums[i]-1) as usize;
                nums[idx] = -1;
            }
        }
        */
        for i in 0..len {
            let mut p_int = nums[i] - 1;
            while 0 <= p_int && p_int < len as i32 {
                let temp = nums[p_int as usize] - 1; 
                nums[p_int as usize] = -1;
                p_int = temp; 
            }
        }
        for i in 0..len {
            if nums[i] > 0 {
                return (i+1) as i32;
            }
        }
        return (len + 1) as i32;
    }
}

pub fn main() {
    let mut nums = vec![1,1];    
    println!("{:?}", nums);
    let ans = Solution::first_missing_positive(nums);
    println!("ans: {:?}", ans);

    let mut nums = vec![3,4,-1,1];    
    println!("{:?}", nums);
    let ans = Solution::first_missing_positive(nums);
    println!("ans: {:?}", ans);

    let mut nums = vec![7,8,9,11,12];
    println!("{:?}", nums);
    let ans = Solution::first_missing_positive(nums);
    println!("ans: {:?}", ans);
}

