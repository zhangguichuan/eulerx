/*
给你一个 m 行 n 列的矩阵 matrix ，请按照 顺时针螺旋顺序 ，返回矩阵中的所有元素。

输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
输出：[1,2,3,6,9,8,7,4,5]

输入：matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
输出：[1,2,3,4,8,12,11,10,9,5,6,7]
*/

struct Solution;
impl Solution {

    pub fn spiral_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        if mat.len() == 0 {
            return vec![];
        }
        let mut ans = vec![];
        let (m, n) = (mat.len() as i32, mat[0].len() as i32);
        let (mut up, mut dw, mut lt, mut rt) = (0, m-1, 0, n-1);
        while true {
            //println!("{w},{h},{r},{c},direction: {up},{dw},{lt},{rt}");
            for k in lt..=rt {
                ans.push(mat[up as usize][k as usize]);
            }
            up+=1;
            if up > dw { break; }

            for k in up..=dw {
                ans.push(mat[k as usize][rt as usize]);
            }
            rt-=1;
            if lt > rt { break; }

            for k in (lt..=rt).rev() {
                ans.push(mat[dw as usize][k as usize]);
            }
            dw-=1;
            if up > dw { break; }

            for k in (up..=dw).rev() {
                ans.push(mat[k as usize][lt as usize]);
            }
            lt+=1;
            if lt > rt { break; }

            //println!("{w},{h},{r},{c},direction: {up},{dw},{lt},{rt}");
        }
        ans
    }

    pub fn spiral_order4(mat: Vec<Vec<i32>>) -> Vec<i32> {
        if mat.len() == 0 {
            return vec![];
        }
        let (m, n) = (mat.len() as i32, mat[0].len() as i32);
        let (mut up, mut dw, mut lt, mut rt) = (0, m-1, 0, n-1);
        let mut ans = vec![];
        while (ans.len() as i32) < m*n {
            let w = n-lt-(n-rt);
            let h = m-up-(m-dw);
            let (mut r, mut c) = (lt as usize,lt as usize);
            if w == 0 && h == 0 {
                ans.push(mat[r][c]);
                break;
            }
            //println!("{w},{h},{r},{c}");
            for k in 0..w {
                ans.push(mat[r][c]);
                c+=1;
            }
            up+=1;

            for k in 0..h {
                ans.push(mat[r][c]);
                r+=1;
            }
            rt-=1;

            for k in 0..w {
                ans.push(mat[r][c]);
                c-=1;
            }
            dw-=1;

            for k in 0..h {
                ans.push(mat[r][c]);
                r-=1;
            }
            lt+=1;
            if lt > rt || up > dw {
                break;
            }
        }
        ans
    }

    //思路不对，m,n的矩阵不能用对角线坐标进行变换
    pub fn spiral_order3(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut ans = vec![];
        let mut width = n;
        let mut h = m;
        for i in 0..n/2+1 {
            width -= 2*i;
            h -= 2*i;
            let (mut r, mut c) = (i, i);
            if width - 1 == 0 && h - 1 == 0 {
                ans.push(mat[i][i]);
                break;
            }
            if width - 1 == 0 && h > 0 {
                for k in 0..h-1 {
                    ans.push(mat[i+k][i]);
                }
                break;
            }
            if width > 0 && h - 1 == 0 {
                for k in 0..width-1 {
                    ans.push(mat[i][i+k]);
                }
                break;
            }
            for j in 0..4 {
                let rr = r;
                let cc = c; 
                println!("{rr},{cc},{width},{h}");
                if j == 0 {
                    for k in 0..width-1 {
                        ans.push(mat[rr][cc+k]);
                    }
                    c = n-1-rr;
                }
                if j == 1 {
                    for k in 0..h-1 {
                        ans.push(mat[rr+k][cc]);
                    }
                    c = h-1-rr;
                }
                if j == 2 {
                    for k in 0..width-1 {
                        ans.push(mat[rr][cc-k]);
                    }
                    c = h-1-rr;
                }
                if j == 3 {
                    for k in 0..h-1 {
                        ans.push(mat[rr-k][cc]);
                    }
                }
                r = cc;
            }
        }
        
        ans
    }

    // 正方形的时候可以使用
    pub fn spiral_order2(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut ans = vec![];
        let mut width = n;
        for i in 0..n/2+1 {
            width -= 2*i;
            let (mut r, mut c) = (i, i);
            if width - 1 == 0 {
                ans.push(mat[i][i]);
                continue;
            }
            for j in 0..4 {
                let rr = r;
                let cc = c; 
                println!("{rr},{cc},{width}");
                for k in 0..width-1 {
                    if j == 0 {
                        ans.push(mat[rr][cc+k]);
                    }
                    if j == 1 {
                        ans.push(mat[rr+k][cc]);
                    }
                    if j == 2 {
                        ans.push(mat[rr][cc-k]);
                    }
                    if j == 3 {
                        ans.push(mat[rr-k][cc]);
                    }
                }
                r = cc;
                c = n-1-rr;
            }
        }
        
        ans
    }
}

pub fn main() {
    let mat = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    println!("{:?}", mat);
    let ans = Solution::spiral_order(mat);
    println!("ans: {:?}", ans);

    let mat = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]];
    println!("{:?}", mat);
    let ans = Solution::spiral_order(mat);
    println!("ans: {:?}", ans);

    let mat = vec![vec![6,9,7]];
    println!("mat: {:?}", mat);
    let ans = Solution::spiral_order(mat);
    println!("ans: {:?}", ans);
}
