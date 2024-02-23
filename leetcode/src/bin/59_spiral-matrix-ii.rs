/*
给你一个正整数 n ，生成一个包含 1 到 n2 所有元素，且元素按顺时针顺序螺旋排列的 n x n 正方形矩阵 matrix 。
示例 1：
输入：n = 3
输出：[[1,2,3],[8,9,4],[7,6,5]]
示例 2：
输入：n = 1
输出：[[1]]
*/

struct Solution;
impl Solution {
    //重点：1.环的层数，用每层的长度代替。可以计算出环的方向的长度
    //2. 对角线是起始位置
    //3. 起始点的坐标变化：上一个方向的行数r等于下一个方向的列：cc=n-1-r, rr=
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    
    }
    //没掌握到精髓的写法 
    pub fn generate_matrix2(n: i32) -> Vec<Vec<i32>> {
        let mut mat: Vec<Vec<i32>> = vec![vec![0;n as usize];n as usize]; 
        let (mut up, mut right, mut down, mut left) = (0,(n-1) as usize,(n-1) as usize,0);
        let (mut i, mut j) = (0,0);
        let (mut k, len) = (1, n*n);
        while k <= n {
            for idx in left..right {
                mat[i][idx] = k*k; 
                k+=1;
            }
            j = right;
            for idx in up..down {
                mat[idx][j] = k*k; 
                k+=1;
            }
            i = down;
            for j in (left+1..=right).rev() {
                mat[i][j] = k*k; 
                k+=1;
            }
            j = left; 
            for i in (up+1..=down).rev() {
                mat[i][j] = k*k; 
                k+=1;
            }
            i = up;
            up+=1;
            down-=1;
            left+=1;
            right -= 1;
        }
        mat
    }
}

pub fn main() {
    let n = 3;
    println!("{:?}", n);
    let ans = Solution::generate_matrix(n);
    println!("ans: {:?}", ans);
}
