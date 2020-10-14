pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack: Vec<i32> = vec![];
    let mut popped_copy = popped.clone();

    let mut pushed_copy = pushed.clone();
    pushed_copy.reverse();

    while popped_copy.len() > 0 {
        if let Some(current_stack_head) = stack.first() {
            if let Some(current_popped_head) = popped_copy.first() {
                if current_popped_head == current_stack_head {
                    // println!("Removing: {}", current_stack_head);
                    popped_copy.remove(0);
                    stack.remove(0);
                } else if pushed_copy.is_empty() {
                    // println!("Breaking");
                    break;
                } else if let Some(current_push_head) = pushed_copy.pop() {
                    stack.insert(0, current_push_head);
                }
            }
        } else if let Some(current_push_head) = pushed_copy.pop() {
            stack.insert(0, current_push_head);
        }

        // println!("Current Push {:?}", pushed_copy);
        // println!("Current Pop {:?}", popped_copy);
        // println!("Stack {:?}", stack);
        // println!("");
    }

    // Return if everything could be popped
    popped_copy.len() == 0
}

#[test]
fn passes_example_one() {
    let pushed = vec![1, 2, 3, 4, 5];
    let popped = vec![4, 5, 3, 2, 1];
    assert_eq!(validate_stack_sequences(pushed, popped), true)
}

#[test]
fn passes_example_two() {
    let pushed = vec![1, 2, 3, 4, 5];
    let popped = vec![4, 3, 5, 1, 2];
    assert_eq!(validate_stack_sequences(pushed, popped), false)
}

#[test]
fn passes_example_three() {
    let pushed = vec![2, 1, 0];
    let popped = vec![1, 2, 0];
    assert_eq!(validate_stack_sequences(pushed, popped), true)
}
