pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;

    for i in (0..digits.len()).rev() {
        if digits[i] != 9 {
            digits[i] += 1;
            return digits;
        }
        digits[i] = 0;
        if i == 0 {
            digits.insert(0, 1);
            return digits;
        }
    }

    digits
}

#[test]
fn test() {
    let digits1 = vec![4, 3, 2, 1];
    let digits2 = vec![1, 2, 3];
    let digits3 = vec![0];
    let digits4 = vec![9];
    let digits5 = vec![1, 9];
    let digits6 = vec![1, 5, 7, 9];
    let digits7 = vec![9, 9];
    let digits8 = vec![1, 9, 9];
    let test_arr: [Vec<i32>; 8] = [
        digits1, digits2, digits3, digits4, digits5, digits6,
        digits7, digits8,
    ];

    for vec in test_arr {
        println!("{:?}", plus_one(vec));
    }
}
