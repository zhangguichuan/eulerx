'''
将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
示例 1：
输入：l1 = [1,2,4], l2 = [1,3,4]
输出：[1,1,2,3,4,4]
示例 2：
输入：l1 = [], l2 = []
输出：[]
示例 3：
输入：l1 = [], l2 = [0]
输出：[0]

'''

from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def mergeTwoLists2(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        if not list1 and not list2:
            return None
        elif not list1:
            return list2
        elif not list2:
            return list1
        else:
            dummyh = tail = ListNode(-1)
            h1,h2 = list1,list2
            while h1 and h2:
                if h1.val <= h2.val:
                    tail.next = h1
                    tail = h1
                    h1 = h1.next
                else:
                    tail.next = h2
                    tail = h2
                    h2 = h2.next
            if not h1:
                tail.next = h2
            if not h2:
                tail.next = h1

            return dummyh.next 
             

    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        if not list1 and not list2:
            return None
        elif not list1:
            return list2
        elif not list2:
            return list1
        else:
            if list1.val < list2.val:
                list1.next = self.mergeTwoLists(list1.next, list2)
                return list1
            else:
                list2.next = self.mergeTwoLists(list1, list2.next)
                return list2

def print_linked_list(node):
    while node:
        print(node.val, end=" -> ")
        node = node.next
    print("None")

if __name__ == "__main__":
    h1 = ListNode(1, ListNode(2, ListNode(4)))
    h2 = ListNode(1, ListNode(3, ListNode(5)))

    #print_linked_list(head);
    s = Solution();
    ans = s.mergeTwoLists(h1, h2)
    print_linked_list(ans);

    h1 = ListNode(5)
    h2 = ListNode(1, ListNode(2, ListNode(4)))
    #print_linked_list(h1);
    s = Solution();
    ans = s.mergeTwoLists(h1, h2)
    print_linked_list(ans);

