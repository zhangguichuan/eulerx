/*
给定一个 n × n 的二维矩阵 matrix 表示一个图像。请你将图像顺时针旋转 90 度。
你必须在 原地 旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要 使用另一个矩阵来旋转图像。
输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
输出：[[7,4,1],[8,5,2],[9,6,3]]

输入：matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
输出：[[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
*/

struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
         
    }
}

pub fn main() {
    let mut nums = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    println!("{:?}", nums);
    Solution::rotate(&mut nums);
    println!("ans: {:?}", nums);
}
