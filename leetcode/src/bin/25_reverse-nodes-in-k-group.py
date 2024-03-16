'''
给你链表的头节点 head ，每 k 个节点一组进行翻转，请你返回修改后的链表。
k 是一个正整数，它的值小于或等于链表的长度。如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。
示例 1：
输入：head = [1,2,3,4,5], k = 2
输出：[2,1,4,3,5]
示例 2：
输入：head = [1,2,3,4,5], k = 3
输出：[3,2,1,4,5]
'''

from typing import Optional

def print_lst(node):
    while node:
        print(node.val, end=" -> ")
        node = node.next
    print("None")

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        if k == 1:
            return head
        tail, nk = head, k - 1
        while nk > 0 and tail:
            tail = tail.next
            nk -= 1
        if nk > 0 or not tail:
            return head
        
        next_h = self.reverseKGroup(tail.next, k)
        pre, nxt = head, head.next
        for _ in range(k-2):
            nxt.next, pre, nxt = pre, nxt, nxt.next
        tail.next = pre

        head.next = next_h

        return tail


    def reverseKGroup_1(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        if k == 1:
            return head
        tail, nk = head, k - 1
        while nk > 0 and tail:
            tail = tail.next
            nk -= 1
        if nk > 0 or not tail:
            return head
        
        next_h = self.reverseKGroup(tail.next, k)
        pre, nxt = head, head.next
        while nxt != tail:
            tmp = nxt.next
            nxt.next = pre

            pre = nxt
            nxt = tmp
        tail.next = pre

        head.next = next_h

        return tail
        

if __name__ == "__main__":
    print(f"k:2")
    h1 = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))
    print_lst(h1)    
    sol = Solution()
    ans = sol.reverseKGroup(h1, 2)
    print_lst(ans)    
        
    print(f"k:3")
    h1 = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))
    print_lst(h1)    
    sol = Solution()
    ans = sol.reverseKGroup(h1, 3)
    print_lst(ans)    

    print(f"k:1")
    h1 = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))
    print_lst(h1)    
    sol = Solution()
    ans = sol.reverseKGroup(h1, 1)
    print_lst(ans)    
        

    print(f"test:")
    # range(0) no print
    for i in range(0):
        print(f"i:{i}")
