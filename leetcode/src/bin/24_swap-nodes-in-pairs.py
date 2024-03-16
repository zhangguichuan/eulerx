'''
给你一个链表，两两交换其中相邻的节点，并返回交换后链表的头节点。你必须在不修改节点内部的值的情况下完成本题（即，只能进行节点交换）。

示例 1：
输入：head = [1,2,3,4]
输出：[2,1,4,3]
示例 2：
输入：head = []
输出：[]
示例 3：
输入：head = [1]
输出：[1]
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
    def swapPairs(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head or not head.next:
            return head
        tail_h = self.swapPairs(head.next.next)
        nh = head.next
        nh.next = head
        head.next = tail_h
        
        return nh

if __name__ == "__main__":
    h1 = ListNode(1, ListNode(2, ListNode(3, ListNode(4))))
    print_lst(h1)    
    sol = Solution()
    ans = sol.swapPairs(h1)
    print_lst(ans)    
        
    h1 = ListNode(1, ListNode(2))
    print_lst(h1)    
    sol = Solution()
    ans = sol.swapPairs(h1)
    print_lst(ans)    
        
    
   

