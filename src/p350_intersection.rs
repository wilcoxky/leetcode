pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut matched: Vec<i32> = vec![];
    let mut n = nums1.clone();
    n.sort();

    let mut m = nums2.clone();
    m.sort();

    for v in n {
        let mut should_remove = false;
        let mut remove_index = 0;
        for (i, v2) in m.iter().enumerate() {
            if v == *v2 {
                matched.push(v);
                should_remove = true;
                remove_index = i;
                break;
            }
        }
        if should_remove {
            m.remove(remove_index);
        }
    }

    matched
}

#[test]
fn passes_example_one() {
    let num1 = vec![1, 2, 3, 4, 5];
    let num2 = vec![4, 5, 3, 2, 1];
    assert_eq!(intersect(num1, num2), [1, 2, 3, 4, 5])
}

#[test]
fn passes_example_two() {
    let num1 = vec![4, 9, 5];
    let num2 = vec![9, 4, 9, 8, 4];
    assert_eq!(intersect(num1, num2), [4, 9])
}

#[test]
fn passes_example_three() {
    let num1 = vec![1, 2, 2, 1];
    let num2 = vec![2, 2];
    assert_eq!(intersect(num1, num2), [2, 2])
}
