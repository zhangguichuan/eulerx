/*
给定一个 m x n 的矩阵，如果一个元素为 0 ，则将其所在行和列的所有元素都设为 0 。请使用 原地 算法。

输入：matrix = [[1,1,1],[1,0,1],[1,1,1]]
输出：[[1,0,1],[0,0,0],[1,0,1]]

输入：matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
输出：[[0,0,0,0],[0,4,5,0],[0,3,1,0]]
*/

struct Solution;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut rows, mut cols) = (vec![false; m], vec![false; n]);
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    rows[i] = true;
                    cols[j] = true;
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if rows[i] == true || cols[j] == true {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

pub fn main() {
    let mut nums: Vec<Vec<i32>> = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
    println!("{:?}", nums);
    Solution::set_zeroes(&mut nums);
    println!("ans: {:?}", nums);

    let mut nums: Vec<Vec<i32>> = vec![vec![0,1,2,0],vec![3,4,5,2],vec![1,3,1,5]];
    println!("{:?}", nums);
    Solution::set_zeroes(&mut nums);
    println!("ans: {:?}", nums);
}
