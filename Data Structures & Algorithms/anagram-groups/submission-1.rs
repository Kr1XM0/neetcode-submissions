impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut list_anagrams = strs.clone();

    while list_anagrams.len() > 0 {
        let mut group: Vec<String> = vec![list_anagrams[0].clone()];

        for x in 1..list_anagrams.len() {
            let mut sort_firts: Vec<char> = list_anagrams[0].chars().collect();
            sort_firts.sort();
            let mut sort_second: Vec<char> = list_anagrams[x].chars().collect();
            sort_second.sort();

            if sort_firts == sort_second {
                group.push(list_anagrams[x].clone());
            }
        }
        result.push(group.clone());
        for item in &group {
            if let Some(pos) = list_anagrams.iter().position(|x| x == item) {
                list_anagrams.remove(pos);
            }
        }
    }
    result
    }
}