#[test]
fn three_sum_closet() {
    assert_eq!(2, crate::demo_algorithm::Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    assert_eq!(0, crate::demo_algorithm::Solution::three_sum_closest(vec![0, 0, 0], 1))
}

#[test]
fn number_of_right_triangles() {
    assert_eq!(2, crate::demo_algorithm::Solution::number_of_right_triangles(vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]]));
}