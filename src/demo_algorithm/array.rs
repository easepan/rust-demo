use crate::demo_algorithm::Solution;

impl Solution {
    /// LeetCode(id=3151,title=特殊数组 I,difficulty=easy)
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut pre = nums[0] & 1;
        for i in 1..nums.len() {
            let odd = nums[i] & 1;
            if odd == pre {
                return false;
            }
            pre = odd;
        }
        return true;
    }

    /// LeetCode(id=3128,title=直角三角形,difficulty=medium)
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let mut row = vec![0; grid.len()];
        let mut col = vec![0; grid[0].len()];
        for (y, arr) in grid.iter().enumerate() {
            for (x, v) in arr.iter().enumerate() {
                col[x] += v;
                row[y] += v;
            }
        }
        let mut res: i64 = 0;
        for (y, &ref arr) in grid.iter().enumerate() {
            for (x, &v) in arr.iter().enumerate() {
                if v == 1 {
                    res += (col[x] - 1) as i64 * (row[y] - 1) as i64
                }
            }
        }
        res
    }

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