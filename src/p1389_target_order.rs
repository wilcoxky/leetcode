pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    if nums.len() != index.len() {
        panic!("Array lengths do not equal");
    }
    let mut target = vec![];

    for (v, i) in nums.iter().zip(index.iter()) {
        target.insert(*i as usize, *v);
    }

    target 
}

// Input: nums = [0,1,2,3,4], index = [0,1,2,2,1]
// Output: [0,4,1,3,2]
// Explanation:
// nums       index     target
// 0            0        [0]
// 1            1        [0,1]
// 2            2        [0,1,2]
// 3            2        [0,1,3,2]
// 4            1        [0,4,1,3,2]
#[test]
fn passes_example_one() {
    let nums = vec![0, 1, 2, 3, 4];
    let index = vec![0, 1, 2, 2, 1];
    let solution = vec![0, 4, 1, 3, 2];
    assert_eq!(create_target_array(nums, index), solution);
}

// Input: nums = [1,2,3,4,0], index = [0,1,2,3,0]
// Output: [0,1,2,3,4]
// Explanation:
// nums       index     target
// 1            0        [1]
// 2            1        [1,2]
// 3            2        [1,2,3]
// 4            3        [1,2,3,4]
// 0            0        [0,1,2,3,4]
#[test]
fn passes_example_two() {
    let nums = vec![1, 2, 3, 4, 0];
    let index = vec![0, 1, 2, 3, 0];
    let solution = vec![0, 1, 2, 3, 4];
    assert_eq!(create_target_array(nums, index), solution);
}

#[test]
fn passes_example_three() {
    let nums = vec![1];
    let index = vec![0];
    let solution = vec![1];
    assert_eq!(create_target_array(nums, index), solution);
}
