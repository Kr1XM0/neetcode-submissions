impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
    for num in 0..nums.len() {
        for x in num + 1..nums.len() {
            if nums[num] == nums[x] {
                return true;
            }
        }
    }
    false
    }
}
