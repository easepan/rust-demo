use crate::demo_algorithm::Solution;

impl Solution {
    /// LeetCode(id=16,title=最接近的三数之和,difficulty=medium)
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let stop1 = nums.len() - 2;
        let stop2 = nums.len() - 1;
        let stop3 = nums.len();
        let mut result = 0;
        let mut diff = i32::MAX;
        for i1 in 0..stop1 {
            let start2 = i1 + 1;
            for i2 in start2..stop2 {
                let start3 = i2 + 1;
                for i3 in start3..stop3 {
                    let sum = nums[i1] + nums[i2] + nums[i3];
                    let sum_diff = (sum - target).abs();
                    if sum_diff < diff {
                        diff = sum_diff;
                        result = sum;
                    }
                }
            }
        }
        return result;
    }
}