fn smallest_compression_removal(s: String, k: i32) -> i32 {
    let mut smallest = i32::max_value();

    for (index, _ch) in s.chars().enumerate() {
        let removed_to_index = index + k as usize;
        if removed_to_index > s.len() { break; }

        let mut substring = s.clone();
        substring.replace_range(index..removed_to_index, "");
        let compressed_len = compress(substring).len() as i32;
        if compressed_len < smallest {
            smallest = compressed_len;
        }
    }
    smallest
}

fn compress(s: String) -> String {
    let mut compressed = String::new();
    let mut count = 0;
    let mut prev_char: char = '1';

    for (index, ch) in s.chars().enumerate() {
        if ch != prev_char && count != 0 {
            // Add on sequence
            let count_string = if count > 1 {
                count.to_string()
            } else {
                "".to_owned()
            };
            let string_to_add = format!("{}{}", count_string, prev_char);
            compressed.push_str(&string_to_add);
            // Set new char to current
            prev_char = ch;
            count = 1;
        } else if count == 0 {
            count += 1;
            prev_char = ch;
        } else if index == s.len() - 1 {
            count += 1;
            let string_to_add = format!("{}{}", count, prev_char);
            compressed.push_str(&string_to_add);
        } else {
            count += 1;
        }
        // compressed
    }
    // Handle left over
    if s.len() == 1 {
        s
    } else {
        compressed
    }
}

#[ignore]
#[test]
fn simple_test() {
    let s = "AAABAAA".to_owned();
    assert_eq!(smallest_compression_removal(s, 1), 2);
}

#[ignore]
#[test]
fn simple_harder() {
    let s = "AAAABBBCCCAAAA".to_owned();
    assert_eq!(smallest_compression_removal(s, 3), 6);
}

#[ignore]
#[test]
fn compression_works() {
    let s = "AAABAAA".to_owned();
    assert_eq!(compress(s), "3AB3A");
}

#[ignore]
#[test]
fn compression_works_small() {
    let s = "A".to_owned();
    assert_eq!(compress(s), "A");
}

#[test]
fn simple_harder_and_long() {
    let s = "ABCDEFGHHHHHHHIWASJNVAS".to_owned();
    assert_eq!(smallest_compression_removal(s, 3), 14);
}

#[test]
fn simple_harder_and_long2() {
    let s = "SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS\
    SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS\
    SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS\
    SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS\
    ABCDEFGHHHHHHHIWASJNVAS".to_owned();
    assert_eq!(smallest_compression_removal(s, 5), 18);
}