use crate::ListNode;
pub fn listnode_to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(n) = node {
        result.push(n.val);
        node = n.next;
    }
    result
}