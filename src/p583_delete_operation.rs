use std::cmp::max;

pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut longest = 0;
    let mut longest_sequence: Vec<Vec<usize>> = vec![vec![0; word2.len() + 1]; word1.len() + 1];
    for (i, c) in word1.chars().enumerate() {
        for (j, compare_c) in word2.chars().enumerate() {
            if i == 0 || j == 0 {
                longest_sequence[i][j] = 0;
            }

            let i_fixed = i + 1;
            let j_fixed = j + 1;
            if c == compare_c {
                let value = longest_sequence[i_fixed - 1][j_fixed - 1] + 1;
                longest_sequence[i_fixed][j_fixed] = value;
                if value > longest {
                    longest = value;
                }
            } else {
                longest_sequence[i_fixed][j_fixed] = max(
                    longest_sequence[i_fixed - 1][j_fixed],
                    longest_sequence[i_fixed][j_fixed - 1],
                );
            }
        }
    }

    let operations: i32 = (word1.len() - longest + word2.len() - longest) as i32;
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

#[test]
fn passes_long_example() {
    let word1 = "setafgry".to_owned();
    let word2 = "eta".to_owned();
    assert_eq!(min_distance(word1, word2), 5);
}

#[test]
fn passes_long_example_harder() {
    let word1 = "seyfgrta".to_owned();
    let word2 = "eta".to_owned();
    assert_eq!(min_distance(word1, word2), 5);
}

#[test]
fn passes_empty_test() {
    let word1 = "a".to_owned();
    let word2 = "".to_owned();
    assert_eq!(min_distance(word1, word2), 1);
}

#[test]
fn passes_none_test() {
    let word1 = "a".to_owned();
    let word2 = "a".to_owned();
    assert_eq!(min_distance(word1, word2), 0);
}

#[test]
fn passes_multiple_entries() {
    let word1 = "intention".to_owned();
    let word2 = "execution".to_owned();
    assert_eq!(min_distance(word1, word2), 8);
}

#[test]
fn passes_multiple_entries_idk() {
    let word1 = "plasma".to_owned();
    let word2 = "altruism".to_owned();
    assert_eq!(min_distance(word1, word2), 8);
}

#[test]
fn passes_insano() {
    let word1 = "zoologicoarchaeologist".to_owned();
    let word2 = "zoogeologist".to_owned();
    assert_eq!(min_distance(word1, word2), 10);
}
