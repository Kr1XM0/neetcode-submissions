impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_silce: Vec<char> = s.chars().collect();
    s_silce.sort();
    let mut t_silce: Vec<char> = t.chars().collect();
    t_silce.sort();

    if s_silce == t_silce {
        return true;
    }
    false
    }
}
