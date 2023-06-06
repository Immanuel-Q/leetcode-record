pub fn is_palindrome(x: i32) -> bool {
    let total = x.to_string();
    let len = total.len();

    let after_start =
        if len % 2 == 0 { len / 2 } else { len / 2 + 1 };

    let prev = &total[..(len / 2)];
    let after = &total[after_start..];
    for (item, item_2) in prev.chars().zip(after.chars().rev()) {
        if item != item_2 {
            return false;
        }
    }

    true
}

pub fn is_palindrome_in_num(x: i32) -> bool {
    let mut n = x;
    let mut rev = 0;

    while n > 0 {
        let digit = n % 10;
        rev = rev * 10 + digit;
        n /= 10;
    }

    x == rev
}

#[test]
fn test() {
    let res = is_palindrome_in_num(4232324);
    println!("{}", res);
}
