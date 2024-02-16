/*
给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word 存在于网格中，返回 true ；否则，返回 false 。
单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。

输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
输出：true
*/

struct Solution;
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {

        pub fn dfs(board: &Vec<Vec<char>>, visited: &mut Vec<Vec<i32>>, m: i32, n: i32, r: i32, c: i32, word: &str, path: &mut String, k: usize) -> bool {
            if path.len() > word.len() {
                return false;
            }
            if path.chars().nth(k) != word.chars().nth(k) {
                return false;
            }
            //println!("path: {:?}", path);
            if word.len() == k+1 {
                return true;
            }
            //这里可以修改为4次或并行，这样会有shortcut短路执行
            for d in vec![(-1,0),(0,1),(1,0),(0,-1)] {
                let (nr, nc) = (r+d.0, c+d.1);
                if nr < 0 || nr >= m {
                    continue ;
                }
                if nc < 0 || nc >= n {
                    continue ;
                }
                if visited[nr as usize ][nc as usize] == 1 {
                    continue ;
                }
                let ch = board[nr as usize][nc as usize];
                visited[nr as usize ][nc as usize] = 1;
                path.push(ch);
                if dfs(board, visited, m, n, nr, nc, word, path, k + 1) {
                    return true;
                }
                path.pop();
                visited[nr as usize ][nc as usize] = 0;
            }
            false
        }

        let mut path = String::from("");
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        let mut visited = vec![];

        for _ in 0..m {
            let mut arr = Vec::new();
            for _ in 0..n {
                arr.push(0); 
            }
            visited.push(arr);
        }

        for i in 0..m {
            for j in 0..n {
                visited[i as usize][j as usize] = 1;
                path.push(board[i as usize][j as usize]);
                if dfs(&board, &mut visited, m, n, i, j, &word, &mut path, 0) {
                    return true;
                }
                path.pop();
                visited[i as usize][j as usize] = 0;
            }
        }
        false
    }
}

pub fn main() {
    let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']]; 
    let word = String::from("ABCCED");
    let is_exist = Solution::exist(board, word);
    println!("is_exist: {}", is_exist);
    let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']]; 
    let word = String::from("ABCB");
    let is_exist = Solution::exist(board, word);
    println!("is_exist: {}", is_exist);
}
