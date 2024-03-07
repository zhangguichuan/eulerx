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
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head == None || head.as_ref().unwrap().next == None {
            return true;
        }
        let mut deque: std::collections::VecDeque<i32> = std::collections::VecDeque::new();
        while head != None {
            let mut tmp = head.take().unwrap();
            deque.push_back(tmp.val);
            head = tmp.next.take();
        }
        while !deque.is_empty() {
            let f = deque.pop_front();
            let b = deque.pop_back();
            if f == None || b == None {
                return true;
            }
            if f != b {
                return false;
            }
        }
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

    //输入：head = [1,2,2,3]
    let mut head = Box::new(ListNode::new(1));
    head.next = Some(Box::new(ListNode::new(2)));
    head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    println!("{:?}", head);
    let ans = Solution::is_palindrome(Some(head));
    println!("ans: {:?}", ans);

    //输入：head = [1,2,1]
    let mut head = Box::new(ListNode::new(1));
    println!("{:?}", head);
    let ans = Solution::is_palindrome(Some(head));
    println!("ans: {:?}", ans);
}

