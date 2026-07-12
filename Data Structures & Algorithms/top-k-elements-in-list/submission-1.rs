impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut nums_map = HashMap::new();
    let mut result = Vec::new();

    for num in nums {
        *nums_map.entry(num).or_insert(0) += 1;
    }

    let mut sort_maps = nums_map.into_iter().collect::<Vec<(i32, i32)>>();
    sort_maps.sort_by(|(_, a), (_, b)| b.cmp(a));

    for x in 0..k {
        result.push(sort_maps[x as usize].0);
    }
    result
    }
}
