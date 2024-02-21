/*
以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。
示例 1：
输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
输出：[[1,6],[8,10],[15,18]]
解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
示例 2：
输入：intervals = [[1,4],[4,5]]
输出：[[1,5]]
解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。
*/

struct Solution;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut ans = vec![]; 
        let (mut l,mut r) = (intervals[0][0], intervals[0][1]);
        for i in 1..intervals.len() {
            if intervals[i][0] <= r {
                r = r.max(intervals[i][1]);
                continue ;
            }
            ans.push(vec![l, r]);
            l = intervals[i][0];
            r = intervals[i][1];
        }
        ans.push(vec![l, r]);
        ans
    }
}

pub fn main() {
    //一样可以做
    //let intervals: Vec<Vec<i32>> = Vec::from([vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]);
    let intervals: Vec<Vec<i32>> = vec![vec![1, 3], vec![2, 6], vec![15, 18], vec![8, 10]];
    println!("{:?}", intervals);
    let ret = Solution::merge(intervals);
    println!("{:?}", ret);

    let intervals: Vec<Vec<i32>> = vec![vec![1, 4], vec![2,3]];
    println!("{:?}", intervals);
    let ret = Solution::merge(intervals);
    println!("{:?}", ret);
}


pub const START: i32 = 1;
pub const END: i32 = 0;

pub struct Point {
    val: i32,
    _type: i32,
}

impl Solution {
    //官方用stack来做，思路也非常的好！
    //先把边界拆开
    pub fn merge_x(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            use std::cmp::Ordering;

    let mut points = vec![];
    for i in intervals {
        points.push(Point {
            val: i[0],
            _type: START,
        });
        points.push(Point {
            val: i[1],
            _type: END,
        });
    }
    points.sort_by(|a, b| {
        if a.val != b.val {
            a.val.cmp(&b.val)
        } else {
            if a._type == END && b._type == START {
                return Ordering::Greater;
            }
            if a._type == START && b._type == END {
                return Ordering::Less;
            }
            Ordering::Less
        }
    });

    let mut ret: Vec<Vec<i32>> = vec![];
    let mut stack = vec![];
    for point in points {
        if point._type == START {
            stack.push(point.val);
        }
        if point._type == END {
            let start = stack.pop().unwrap();
            if stack.is_empty() {
                ret.push(vec![start, point.val]);
            }
        }
    }

    ret
    }
}
