use std::collections::HashMap;

pub fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut hmap = HashMap::with_capacity(nums.len());
    for (i, item) in nums.iter().enumerate() {
        if let Some(k) = hmap.get(&(target - item)) {
            if *k != i {
                return vec![*k as i32, i as i32];
            }
        }
        hmap.insert(item, i);
    }
    panic!("not Found");
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test() {
        let nums = vec![3, 3]; // (3, 0), (3, 1);

        // let nums = vec![2, 7, 11, 15];

        let sum = two_sum(&nums, 6);
        println!("{:?}", sum);
    }
}
