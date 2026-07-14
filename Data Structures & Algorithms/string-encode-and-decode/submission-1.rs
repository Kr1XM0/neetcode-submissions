impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
    let res: String = strs
        .into_iter()
        .map(|word| format!("{}#{}", word.len(), word))
        .collect();
    res
    }

    pub fn decode(s: String) -> Vec<String> {
    let s_chars: Vec<char> = s.chars().collect();
    let mut init: usize = 0;
    let mut result: Vec<String> = Vec::new();

    while init < s_chars.len() {
        let mut j = init;
        while s_chars[j] != '#' {
            j += 1;
        }
        let len_str: String = s_chars[init..j].iter().collect();
        let length: usize = len_str.parse().unwrap();
        
        let word: String = s_chars[j + 1..j + 1 + length].iter().collect();
        result.push(word);
        init = j + 1 + length;
    }

    result
    }
}