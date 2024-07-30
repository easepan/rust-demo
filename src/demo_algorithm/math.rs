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
}