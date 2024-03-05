/*
给你一个单链表的头节点 head ，请你判断该链表是否为回文链表。如果是，返回 true ；否则，返回 false 。
输入：head = [1,2,2,1]
输出：true

输入：head = [1,2]
输出：false
*/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

struct Solution;
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {

        true
    }
}

pub fn main() {
    //输入：head = [1,2,2,1]
    let mut head = Box::new(ListNode::new(1));
    head.next = Some(Box::new(ListNode::new(2)));
    head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
    println!("{:?}", head);
    let ans = Solution::is_palindrome(Some(head));
    println!("ans: {:?}", ans);
}

