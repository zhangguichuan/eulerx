"""
给你两个单链表的头节点 headA 和 headB ，请你找出并返回两个单链表相交的起始节点。如果两个链表不存在相交节点，返回 null 。

图示两个链表在节点 c1 开始相交：

题目数据 保证 整个链式结构中不存在环。

注意，函数返回结果后，链表必须 保持其原始结构 。
"""
from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    # 更好的写法
    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        h1,h2 = headA,headB
        while h1 != h2:
            h1 = h1.next if h1 else headB
            h2 = h2.next if h2 else headA
        return h1
        
    # id
    def getIntersectionNode2(self, headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        h1,h2 = headA,headB
        while id(h1) != id(h2):
            if h1 is None:
                h1 = headB
                h2 = h2.next
                continue ;
            if h2 is None:
                h2 = headA
                h1 = h1.next
                continue
            h1 = h1.next
            h2 = h2.next
        return h1

if __name__ == "__main__":
    c = ListNode(8)
    c.next = ListNode(4)
    c.next.next = ListNode(5)
    print(f"common id is: {id(c)}")

    head1 = ListNode(4)
    head1.next = ListNode(1)
    head1.next.next = c

    head2 = ListNode(5)
    head2.next = ListNode(6)
    head2.next.next = ListNode(1)
    head2.next.next.next = c
    sol = Solution();
    ans = sol.getIntersectionNode(head1, head2);
    print(f"{ans.val},{id(ans)}")
    
    c = ListNode(3)
    print(f"common id is: {id(c)}")

    head1 = c

    head2 = ListNode(2)
    head2.next = c
    sol = Solution();
    ans = sol.getIntersectionNode(head1, head2);
    print(f"{None if ans == None else  ans.val},{id(ans)}")
    

