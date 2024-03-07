'''
给你一个链表的头节点 head ，判断链表中是否有环。
如果链表中有某个节点，可以通过连续跟踪 next 指针再次到达，则链表中存在环。 为了表示给定链表中的环，评测系统内部使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。注意：pos 不作为参数进行传递 。仅仅是为了标识链表的实际情况。
如果链表中存在环 ，则返回 true 。 否则，返回 false 。

输入：head = [3,2,0,-4], pos = 1
输出：true
解释：链表中有一个环，其尾部连接到第二个节点。
输入：head = [1,2], pos = 0
输出：true
解释：链表中有一个环，其尾部连接到第一个节点。
输入：head = [1], pos = -1
输出：false
解释：链表中没有环。
'''

from typing import Optional
# Definition for singly-linked list.
class ListNode:
    def __init__(self, x, nxt=None):
        self.val = x
        self.next = nxt

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        if not head or not head.next:
            return False
        slow = head
        fast = head.next
        while slow != fast:
            if not slow or not fast or not fast.next:
                return False
            slow = slow.next
            fast = fast.next.next
             
        return True

    def hasCycle2(self, head: Optional[ListNode]) -> bool:
        if head is None or head.next is None:
            return False
        slow = head
        fast = head.next
        while fast != slow and fast is not None and slow is not None:
            slow = slow.next
            fast = fast.next
            if fast is None:
                return False
            fast = fast.next

        if fast == None or slow == None:
            return False

        return True

def print_linked_list(node):
    while node:
        print(node.val, end=" -> ")
        node = node.next
    print("None")


if __name__ == "__main__":
    # head = [3,2,0,-4], pos = 1

    head = tail = ListNode(3)
    ring_node = ListNode(2)
    tail.next = ring_node
    tail = ring_node
    tail.next = ListNode(0)
    tail = tail.next
    tail.next = ListNode(-4)
    tail = tail.next
    tail.next = ring_node
    
    # print_linked_list(head);
    s = Solution();
    ans = s.hasCycle(head)
    print(f"1: {ans}");
    
    head = ListNode(3)
    print(f"{head}");
    s = Solution();
    ans = s.hasCycle(head)
    print(f"2: {ans}");

    head = ListNode(3, ListNode(4))
    print(f"{head}");
    s = Solution();
    ans = s.hasCycle(head)
    print(f"3: {ans}");

    pass
    
