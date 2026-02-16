/*
You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself. 
*/
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0); // Start with a dummy node
    let mut tail = &mut dummy_head;        // Tail pointer for building the result
    let mut carry = 0;                     // Carry for addition
    // Loop while either list has nodes or there is a carry left
    while l1.is_some() || l2.is_some() || carry != 0 {
        // Get values from current nodes, or 0 if list is empty
        let v1 = l1.as_ref().map_or(0, |node| node.val);
        let v2 = l2.as_ref().map_or(0, |node| node.val);

        // Add values and carry
        let sum = v1 + v2 + carry;
        carry = sum / 10; // Update carry for next digit

        // Create new node with the digit (sum % 10)
        tail.next = Some(Box::new(ListNode::new(sum % 10)));
        tail = tail.next.as_mut().unwrap(); // Move tail forward

        // Move to next nodes in l1 and l2
        l1 = l1.and_then(|node| node.next);
        l2 = l2.and_then(|node| node.next);
    }

    dummy_head.next // Return the result, skipping the dummy node
}