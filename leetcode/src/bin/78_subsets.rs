/*
给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的子集（幂集）。
解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。
示例 1：
输入：nums = [1,2,3]
输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
示例 2：
输入：nums = [0]
输出：[[],[0]]
 */
//1. 递归的dfs写法
//2. 二进制枚举的方式: 使用二进制枚举的方式，真是非常方便，利用整数和二进制的关系提取字符串中的字符
//思考：只有两种状态的序列都可以使用类似的方法
struct Solution;
impl Solution {

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let setn = 1<<n; //2^n
        let mut ret = vec![];
        for i in 0..setn {
            let mut set = vec![];
            for k in 0..n {
                let is_choose = (i >> k) & 0x1;
                if is_choose == 1 {
                    set.push(nums[k]);
                }
            }
            ret.push(set);
        }
        //println!("{:?}", ret);
        ret
    }
    pub fn subsets2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = vec![]; 
        //let dfs = |nums: &Vec<i32>, cur: usize, set: Vec<i32>| {
        //}
        fn dfs(nums: &Vec<i32>, cur: usize, set: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
            if cur >= nums.len() {
                ret.push(set.clone());
                return ;
            }
            
            //no cur value
            dfs(nums, cur+1, set, ret);
            //use cur value
            set.push(nums[cur]);
            dfs(nums, cur+1, set, ret);
            set.pop();
        }
        let mut set: Vec<i32> = vec![]; 
        dfs(&nums, 0, &mut set, &mut ret);
        ret
    }
}

pub fn main() {
    let nums: Vec<i32> = vec![1,2,3];
    println!("{:?}", nums);
    let subsets = Solution::subsets(nums);
    println!("subsets: {:?}", subsets);

}
