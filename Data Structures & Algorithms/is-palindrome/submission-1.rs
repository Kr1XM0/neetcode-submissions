impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let char_vec: String = s.to_lowercase().split_whitespace().collect();
        let char_vec: Vec<char> = char_vec.chars().filter(|x| x.is_alphanumeric()).collect();
        
        let mut rev_char_vec = char_vec.clone();
        rev_char_vec.reverse();

        if char_vec == rev_char_vec {
            return true;
        }
        false
    }
}
