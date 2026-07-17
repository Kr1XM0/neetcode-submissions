impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0
        }
        let mut count = 1;
        let mut high_values = HashSet::new();
        let mut copy_nums = nums;
        copy_nums.sort();

        for (index, num) in copy_nums.iter().enumerate() {
            if index == 0 || *num == copy_nums[index - 1] {
                continue;
            }
            if *num - 1 == copy_nums[index - 1] {
                count += 1
            } else {
                count = 1;
            }
            high_values.insert(count);
        }

        let count = if let Some(value) = high_values.iter().max() {
            *value
        } else {
            count
        };

        count
        }
}
