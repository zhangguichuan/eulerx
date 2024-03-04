/*
给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
示例 1：
输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
输出：6
解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。
示例 2：
输入：height = [4,2,0,3,2,5]
输出：9
*/

struct Solution;
impl Solution {
    //单调栈解法
    pub fn trap(height: Vec<i32>) -> i32 {}

    //这种双指针，就直接先计算出左，右两边的最大值，然后再计算一次，比较优雅
    pub fn trap3(height: Vec<i32>) -> i32 {}

    //这种双指针方法，需要从前向后和从后向前两个半次
    pub fn trap2(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut ret = 0;
        let (mut l, mut r) = (0, 0);
        //from left to right
        while l < len {
            let h = height[l];
            let mut r = l + 1;
            while r < len && height[r] < height[l] {
                r += 1;
            }
            if r < len {
                while l < r {
                    ret += h - height[l];
                    l += 1;
                }
            } else {
                break;
            }
        }
        //from right to left
        let mut r = len - 1;
        while l < r {
            let h = height[r];
            let mut r1 = r - 1;
            while r1 >= l && height[r1] < height[r] {
                r1 -= 1;
            }
            if r1 >= l {
                while r > r1 {
                    ret += h - height[r];
                    r -= 1;
                }
            } else {
                r -= 1;
            }
        }

        ret
    }
}

pub fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!("{:?}", height);
    let ret = Solution::trap(height);
    println!("{:?}", ret);

    let height = vec![4, 2, 0, 3, 2, 5];
    println!("{:?}", height);
    let ret = Solution::trap(height);
    println!("{:?}", ret);

    let height = vec![4, 2, 3];
    println!("{:?}", height);
    let ret = Solution::trap(height);
    println!("{:?}", ret);

    let height = vec![3, 4];
    println!("{:?}", height);
    let ret = Solution::trap(height);
    println!("{:?}", ret);
}
