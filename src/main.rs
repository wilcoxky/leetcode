mod substring;

fn main() {
    println!("Hello, world!");
    let l1 = value_to_list(5);
    let l2 = value_to_list(5);
    // println!("List value = {:?}", get_list_value(l1, 0));
    println!("List value = {:?}", add_two_numbers(l1, l2));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut first_index: i32 = 0;
    let mut second_index: i32 = 0;
    for (index, value) in nums.iter().enumerate() {
        for (next_index, next_value) in nums.iter().enumerate().skip(index + 1) {
            if value + next_value == target {
                first_index = index as i32;
                second_index = next_index as i32;
            }
        }
    }
    vec![first_index, second_index]
}

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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    solve_nth_place_digit(l1, l2, 0, false)
}

fn solve_nth_place_digit(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    depth: i32,
    carryover: bool,
) -> Option<Box<ListNode>> {
    let base = 10;
    match (l1, l2) {
        (Some(n1), Some(n2)) => {
            let added_digits = if carryover {
                n1.val + n2.val + 1
            } else {
                n1.val + n2.val
            };
            let new_value = added_digits % base;
            let mut new_node = Box::new(ListNode::new(new_value));
            new_node.next = solve_nth_place_digit(n1.next, n2.next, depth + 1, added_digits >= 10);
            Some(new_node)
        }
        (Some(n1), None) => {
            let added_digits = if carryover { n1.val + 1 } else { n1.val };
            let new_value = added_digits % base;
            let mut new_node = Box::new(ListNode::new(new_value));
            new_node.next = solve_nth_place_digit(n1.next, None, depth + 1, added_digits >= 10);
            Some(new_node)
        }
        (None, Some(n2)) => {
            let added_digits = if carryover { n2.val + 1 } else { n2.val };
            let new_value = added_digits % base;
            let mut new_node = Box::new(ListNode::new(new_value));
            new_node.next = solve_nth_place_digit(None, n2.next, depth + 1, added_digits >= 10);
            Some(new_node)
        }
        (None, None) => {
            if carryover {
                Some(Box::new(ListNode::new(1)))
            } else {
                None
            }
        }
    }
}

fn value_to_list(value: i32) -> Option<Box<ListNode>> {
    if value == 0 {
        Some(Box::new(ListNode::new(0)))
    } else {
        let base: i32 = 10;
        let mut values = vec![];
        let mut depth = 0;
        let mut divisor = base.pow(depth);
        while divisor <= value {
            let digit = (value / divisor) % base;
            values.push(digit);
            depth += 1;
            divisor = base.pow(depth);
        }
        // Convert to list
        let mut previous_node: Option<Box<ListNode>> = None;
        values.reverse();
        for v in values {
            let mut new_node = Box::new(ListNode::new(v as i32));
            match previous_node {
                Some(node) => {
                    new_node.next = Some(node);
                    previous_node = Some(new_node);
                }
                _ => {
                    previous_node = Some(new_node);
                }
            };
        }
        previous_node
    }
}
