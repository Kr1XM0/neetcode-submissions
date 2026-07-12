impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
        for num in (i + 1)..nums.len() {
            if nums[i] + nums[num] == target {
                return vec![i as i32, num as i32];
            }
        }
    }
    vec![0, 0]
    }
}
