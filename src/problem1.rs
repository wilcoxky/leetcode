
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