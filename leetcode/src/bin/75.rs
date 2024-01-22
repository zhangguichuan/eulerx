/*
 给定一个包含红色、白色和蓝色、共 n 个元素的数组 nums ，原地对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。

我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。

必须在不使用库内置的 sort 函数的情况下解决这个问题。
*/

//思路：1.3个指针，两个指针从两端走，一个指针走扫描，找到0就和左端指针交换；找到2就和右端指针交换
struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut l,mut r, mut i) = (0, nums.len()-1, 0);
        while i < nums.len() {
            if nums[i] == 2 {
                while i < r && nums[r] == 2 { r-=1; }
                //方案0
                if i < r {
                    nums.swap(i,r);
                    r-=1;
                } else {
                    break;
                }
                //方案1: 先break的场景，必须要注意r 可能会小于i,
                //要注意索引交叉的情况，使用i>=r判断异常更好
                //if i>=r {
                //    break;
                //}
                //nums.swap(i,r);
                //r-=1;
                //方案2
                //if i == r {
                //    break;
                //}
                //nums.swap(i, r);
                //r-=1;  //这里不能有，因为可能i>r 无法判断到，如果要打开r-=1;那么前面的i==r 修改为
                //r>=r 条件
            } else if nums[i] == 0 {
                while l < i && nums[l] == 0 { l+=1; }
                if l < i {
                    nums.swap(l, i);
                } else {
                    i+=1;
                    l+=1;
                }
            } else {
                i+=1;
            }
        }
    }
}


pub fn main() {
    let mut arr = vec![2,0,2,1,1,0];
    println!("before: {:?}", arr);
    Solution::sort_colors(&mut arr); 
    println!("sorted arr is: {:?}",arr);
    assert_eq!(vec![0,0,1,1,2,2],arr);

    let mut arr: Vec<i32> = vec![1,2,0];
    println!("before: {:?}", arr);
    Solution::sort_colors(&mut arr); 
    println!("sorted arr is: {:?}",arr);
    assert_eq!(vec![0,1,2],arr);

    let mut arr: Vec<i32> = vec![1,0,0];
    println!("before: {:?}", arr);
    Solution::sort_colors(&mut arr); 
    println!("sorted arr is: {:?}",arr);
    assert_eq!(vec![0,0,1],arr);
}
    
