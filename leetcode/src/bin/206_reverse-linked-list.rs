/*
给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。

输入：head = [1,2,3,4,5]
输出：[5,4,3,2,1]

输入：head = [1,2]
输出：[2,1]
示例 3：

输入：head = []
输出：[]
*/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    //使用栈代替递归
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None || head.as_ref().unwrap().next == None {
            return head;
        }
        let mut sk = vec![];
        while let Some(mut node) = head {
            let next = node.next.take();
            sk.push(node);
            head = next;
        }
        let mut h = sk.pop();
        let mut cur = &mut h;
        while let Some(node) = sk.pop() {
            cur.as_mut().unwrap().next = Some(node);
            cur = &mut cur.as_mut().unwrap().next;
        }
        h
    }

    //标准的官方写法，用两个指针:pre,curr
    pub fn reverse_list3(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None || head.as_ref().unwrap().next == None {
            return head;
        }
        let mut pre = None;
        let mut curr = head;
        while curr != None {
            let next = curr.as_mut().unwrap().next.take();
            curr.as_mut().unwrap().next = pre;
            pre = curr;
            curr = next;
        }
        return pre;
    }

    pub fn reverse_list2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        /*
        //rust用递归翻转链表很麻烦，链表的递归转为栈或者其他的方式执行
        pub fn recur<'a>(h: &'a mut Option<&'a mut Box<ListNode>>) -> &'a Option<&'a mut Box<ListNode>> {
            if h.as_ref().unwrap().next == None {
                return h;
            }
            let ans = recur(&mut h.as_mut().unwrap().next.as_mut());
            h.as_mut().unwrap().next.as_mut().unwrap().next = h;
            h.as_mut().unwrap().next = None;
            return ans
        }
        */

        //非递归方式： 得用3个指针
        pub fn recur(h: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if h == None || h.as_ref().unwrap().next == None {
                return h;
            }
            let mut cur = h;
            let mut next = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = None;
            while next != None {
                let nnext = next.as_mut().unwrap().next.take();
                next.as_mut().unwrap().next = cur;

                cur = next;
                next = nnext;
            }

            return cur;
        }

        let h = recur(head);
        return h;
    }
}

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list_x(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut curr_node) = curr.take() {
            let temp = curr_node.next.take();
            curr_node.next = prev.take();

            prev = Some(curr_node);
            curr = temp;
        }

        prev
    }
}

pub fn main() {
    //输入：head = [1,2,3,4,5]
    let mut head = Box::new(ListNode::new(1));
    head.next = Some(Box::new(ListNode::new(2)));
    head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    head.next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(5)));
    println!("{:?}", head);
    let ans = Solution::reverse_list(Some(head));
    println!("ans: {:?}", ans);
}
