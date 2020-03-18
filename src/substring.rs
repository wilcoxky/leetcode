pub fn length_of_longest_substring(s: String) -> i32 {
    let mut current_chain = Vec::new();
    let mut longest = 0;
    for c in s.chars() {
        // Handle no sequence in chain
        if current_chain.len() == 0 {
            current_chain.push(c);
            longest = current_chain.len();
            continue;
        }

        let current_chain_copy = current_chain.clone();
        current_chain.push(c);

        // Check if current substring contains the current letter
        // if it does remove up to the index of that letter
        for (index, current_char) in current_chain_copy.iter().enumerate() {
            if *current_char == c {
                // println!("Before: {:?}", current_chain);
                let new_len = current_chain.len() - 1;
                longest = if new_len > longest { new_len } else { longest };
                // println!("Index: {}, Lognest {}", index, longest);
                // Pop until current index and add on
                current_chain.drain(..index + 1);
                // println!("After: {:?}", current_chain);
            }
        }
        
        // Update after with new drained
        if current_chain.len() > longest {
            // println!("Current Chain: {:?}", current_chain);
            longest = current_chain.len();
        }
    }
    longest as i32
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_longest_chain() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_owned()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_owned()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_owned()), 3);
    }

    #[test]
    fn hardest_chain_test() {
        assert_eq!(length_of_longest_substring("abcdaefgh".to_owned()), 8);
        assert_eq!(length_of_longest_substring("abcdeabcdf".to_owned()), 6);
    }
}
