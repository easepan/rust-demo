use crate::demo_algorithm::Solution;

const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
const TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
const HUNDREDS: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];

impl Solution {
    /// LeetCode(id=12,title=整数转罗马数字,difficulty=medium)
    pub fn int_to_roman(num: i32) -> String {
        let n = num as usize;
        format!("{}{}{}{}", THOUSANDS[n / 1000], HUNDREDS[n / 100 % 10], TENS[n / 10 % 10], ONES[n % 10])
    }

    /// LeetCode(id=2961,title=双模幂运算,difficulty=medium)
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        let pow = |mut x, mut n, m| {
            // 本题 m 很小，即使平方也不会超过 i32 范围，所以不需要用 i64
            let mut res = 1;
            while n > 0 {
                if n & 1 == 1 {
                    res = res * x % m;
                }
                x = x * x % m;
                n >>= 1;
            }
            res
        };
        let check = |v: &Vec<_>| pow(pow(v[0], v[1], 10), v[2], v[3]) == target;
        variables.iter()
            .enumerate()
            .filter_map(|(i, v)| check(v).then_some(i as i32))
            .collect()
    }
}