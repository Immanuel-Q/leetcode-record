use leetcode_record::simple_part::two_sum;

fn main() {
    let nums = vec![3, 3];

    let sum = two_sum::two_sum(&nums, 6);
    println!("{:?}", sum);
}
