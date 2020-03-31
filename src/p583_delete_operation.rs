pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut operations = 0;

    let mut char_array_word1 = vec![0; 500];
    // Find longest subsequence that matches between the two strings
    // then remove the surrounding elements, but this doesn't work for all
    // solutions
    
    operations
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

#[test]
fn passes_example_one_modified() {
    let word1 = "sea".to_owned();
    let word2 = "eta".to_owned();
    assert_eq!(min_distance(word1, word2), 2)
}

#[test]
fn passes_words_of_dif_len() {
    let word1 = "seat".to_owned();
    let word2 = "eat".to_owned();
    assert_eq!(min_distance(word1, word2), 1)
}

#[test]
fn passes_no_changes() {
    let word1 = "abcdefg".to_owned();
    let word2 = "abcdefg".to_owned();
    assert_eq!(min_distance(word1, word2), 0)
}