pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut operations = 0;

    let mut start_word1 = 0;
    let mut start_word2 = 0;

    let chars_word1 = word1.into_bytes();
    let chars_word2 = word2.into_bytes();

    let mut new_word = "".to_owned();

    while start_word1 < chars_word1.len() {
        let cur_char = chars_word1[start_word1];
        start_word1 += 1;
        let mut matched = false;

        // Keep track of where previous pointer was just in case char not found
        let prev_word2_start = start_word2;

        while !matched && start_word2 < chars_word2.len() {
            let match_char = chars_word2[start_word2];

            // If found, count operations required to remove
            if match_char == cur_char {
                matched = true;
                new_word.push(match_char as char);
                let required_steps = (start_word2 - prev_word2_start) as i32;
                println!("Operations added: {}", required_steps);
                println!("{} to {}", chars_word2[prev_word2_start] as char, match_char as char);
                operations += required_steps;
            }
            // Advance pointer
            start_word2 += 1;

            // Word 1 Char not found in word 2, remove
            if start_word2 == chars_word2.len() {
                start_word2 = prev_word2_start;
                operations += 1;
                break;
            }
        }
    }

    // Add whatever if left of word 2
    let word2_leftover = (chars_word2.len() - start_word2) as i32;
    // operations += word2_leftover;
    println!("New Word: {:?}", new_word);
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


#[test]
fn passes_single_change() {
    let word1 = "abctdefg".to_owned();
    let word2 = "abcdefg".to_owned();
    assert_eq!(min_distance(word1, word2), 1)
}