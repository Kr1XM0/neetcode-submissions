impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .enumerate()
        .map(|(index, num)| {
            let mut amount = 1;
            for (i, x) in nums.iter().enumerate() {
                if x == num && i == index {
                    continue;
                } else {
                    amount *= x
                }
            }
            amount
        })
        .collect::<Vec<i32>>()
    }
}
