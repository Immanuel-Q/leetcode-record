/// convert the roman number to int.
pub fn roman_to_int(s: String) -> i32 {
    const ROMAN: [(char, i32); 7] = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ];

    let mut total = 0;

    let mut temp_vec = vec![];
    for char in s.chars() {
        for item in ROMAN {
            if item.0 == char {
                temp_vec.push(item);
            }
        }
    }

    let mut prev: Option<&i32> = None;
    for (i, (r, v)) in temp_vec.iter().enumerate() {
        if let Some(&prev_num) = prev {
            total += v - prev_num;
            prev = None;
        } else {
            if ['I', 'X', 'C'].contains(r) {
                if let Some((after_r, _)) = temp_vec.get(i + 1) {
                    match r {
                        'I' if ['V', 'X'].contains(after_r) => {
                            prev = Some(v);
                            continue;
                        }
                        'X' if ['L', 'C'].contains(after_r) => {
                            prev = Some(v);
                            continue;
                        }
                        'C' if ['D', 'M'].contains(after_r) => {
                            prev = Some(v);
                            continue;
                        }
                        _ => (),
                    }
                }
            }
            total += v;
        }
    }

    total
}

#[test]
fn test() {
    // let s = String::from("MCMXCIV");
    let s = String::from("III");
    let res = roman_to_int(s);
    println!("{}", res);
}
