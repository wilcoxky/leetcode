pub fn min_distance(word1: String, word2: String) -> i32 {
    unimplemented!();
    let mut char_array_word1 = vec![0; 500];
    let mut char_array_word2 = vec![0; 500]:
    
}


// Input: "sea", "eat"
// Output: 2
// Explanation: You need one step to make "sea" to "ea" and another step to make "eat" to "ea".
#[test]
fn passes_example_one() {
    let word1 = "sea".to_owned();
    let word2 = "eat".to_owned();
    assert_eq!(min_distance(word1, word2), 2)
}