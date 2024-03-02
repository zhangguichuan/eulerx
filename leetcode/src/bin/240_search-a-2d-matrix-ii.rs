/*
编写一个高效的算法来搜索 m x n 矩阵 matrix 中的一个目标值 target 。该矩阵具有以下特性：
每行的元素从左到右升序排列。
每列的元素从上到下升序排列。

输入：matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 5
输出：true

输入：matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 20
输出：false
*/

struct Solution;
impl Solution {
    //右向做旋转90度，形成类查找二叉树结构结构
    pub fn search_matrix(mat: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (mat.len(), mat[0].len());
        //let directions = vec![vec![0,-1],vec![1,0]];
        let (mut r, mut c) = (0 as usize, (n-1) as i32);
        while r < m && c >= 0 {
            if mat[r][c as usize] == target {
                return true;
            } else if target > mat[r][c as usize] {
                r+=1;
            } else {
                c-=1;
            }
        }
        return false;
    }

    pub fn search_matrix3(mat: Vec<Vec<i32>>, target: i32) -> bool {
        pub fn square_search(mat: Vec<Vec<i32>>, target: i32, m: usize, n: usize) -> bool {
            let min = std::cmp::min(m,n);
            let mut i = 0;
            while i < min && target > mat[i][i] {
                i+=1;
            }
            if i < min && target <= mat[i][i] && target >= mat[i][0] {
                let mut k = 0;
                for k in 0..=i {
                    if mat[i][k] == target {
                        return true;
                    }
                }
            }
            if i < min && target <= mat[i][i] && target >= mat[0][i] {
                let mut k = 0;
                for k in 0..=i {
                    if mat[k][i] == target {
                        return true;
                    }
                }
            }
            if min == i && min == m {
                for k in i..n {
                    if mat[m-1][k] == target {
                        return true;
                    }
                }
            }
            if min == i && min == n {
                for k in i..m {
                    if mat[k][n-1] == target {
                        return true;
                    }
                }
            }
            return false;
        }

        //split matrix into square,not easy
        let (m, n) = (mat.len(), mat[0].len());
        let (mut up, mut dw, mut l, mut r) = (0, m, 0, n);
        let mut min = l.min(r);
        while r - l != 1 && l != r {
            
        }
        true
    } 

    //从对角线用二分查找,这种方法只能搜索方阵或者单行/单列矩阵
    pub fn search_matrix2(mat: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (mat.len(), mat[0].len());
        let min = std::cmp::min(m,n);
        let mut i = 0;
        while i < min && target > mat[i][i] {
            i+=1;
        }
        if i < min && target <= mat[i][i] && target >= mat[i][0] {
            let mut k = 0;
            for k in 0..=i {
                if mat[i][k] == target {
                    return true;
                }
            }
        }
        if i < min && target <= mat[i][i] && target >= mat[0][i] {
            let mut k = 0;
            for k in 0..=i {
                if mat[k][i] == target {
                    return true;
                }
            }
        }
        if min == i && min == m {
            for k in i..n {
                if mat[m-1][k] == target {
                    return true;
                }
            }
        }
        if min == i && min == n {
            for k in i..m {
                if mat[k][n-1] == target {
                    return true;
                }
            }
        }
        return false;
    }
}

pub fn main() {
    let mat = vec![vec![1,4,7,11,15],vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]];
    let target = 5;
    println!("{:?}, {}", mat, target);
    let ans = Solution::search_matrix(mat, target);
    println!("ans: {:?}", ans);

    let mat = vec![vec![1,4,7,11,15],vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]];
    let target = 25;
    println!("{:?}, {}", mat, target);
    let ans = Solution::search_matrix(mat, target);
    println!("ans: {:?}", ans);

    let mat = vec![vec![1,4],vec![2,5]];
    let target = 4;
    println!("{:?}, {}", mat, target);
    let ans = Solution::search_matrix(mat, target);
    println!("ans: {:?}", ans);

    let mat = vec![vec![-1,3]];
    let target = 3;
    println!("{:?}, {}", mat, target);
    let ans = Solution::search_matrix(mat, target);
    println!("ans: {:?}", ans);

    let mat = vec![vec![1,2,3,4,5],vec![6,7,8,9,10],vec![11,12,13,14,15],vec![16,17,18,19,20],vec![21,22,23,24,25]];
    let target = 15;
    println!("{:?}, {}", mat, target);
    let ans = Solution::search_matrix(mat, target);
    println!("ans: {:?}", ans);

    let mat = vec![vec![-5]];
    let target = -10;
    println!("{:?}, {}", mat, target);
    let ans = Solution::search_matrix(mat, target);
    println!("ans: {:?}", ans);
}


use std::cmp::Ordering;
impl Solution {
    //j 应该是i32,否则会overflow, 官方代码有问题
    pub fn search_matrix_x(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut i, mut j) = (0, n - 1);

        while i < m && j as i32 >= 0 {
            match matrix[i][j].cmp(&target) {
                Ordering::Less => { i += 1 }
                Ordering::Equal => { return true; }
                Ordering::Greater => { j -= 1 }
            }
        }

        false
    }
}
