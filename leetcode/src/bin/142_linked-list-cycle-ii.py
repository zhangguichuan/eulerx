'''
给定一个链表的头节点  head ，返回链表开始入环的第一个节点。 如果链表无环，则返回 null。

如果链表中有某个节点，可以通过连续跟踪 next 指针再次到达，则链表中存在环。 为了表示给定链表中的环，评测系统内部使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。如果 pos 是 -1，则在该链表中没有环。注意：pos 不作为参数进行传递，仅仅是为了标识链表的实际情况。

不允许修改 链表。

'''

from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, x, nxt=None):
        self.val = x
        self.next = nxt

class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head or not head.next:
            return None
        slow,fast = head,head.next
        while fast != slow:
            if not fast or not slow or not fast.next:
                return None
            slow = slow.next
            fast = fast.next.next
        if not fast or not slow:
            return None
        
        k = 1
        slow = slow.next
        fast = fast.next.next
        while fast != slow:
            k+=1
            slow = slow.next
            fast = fast.next.next
        print(f"k:{k}")
        slow = head
        fast = head
        while k > 0:
            print(f"v:{fast.val}")
            k-=1
            fast = fast.next
        while fast != slow:
            slow = slow.next
            fast = fast.next
        
        return fast
        

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
    ans = s.detectCycle(head)
    print(f"t1: {ans.val}");

    head = ListNode(3)
    print(f"{head}");
    s = Solution();
    ans = s.detectCycle(head)
    print(f"t2: {ans}");

    head = ListNode(1, ListNode(2))
    head.next.next = head
    print(f"{head}");
    s = Solution();
    ans = s.detectCycle(head)
    print(f"t3: {ans.val}");

    pass

