fn check_odd(num: u32) -> bool {
    return num % 2 != 0;
}

fn validate_cc(cc: &str) -> bool {
    let is_cc_odd = check_odd(cc.len() as u32);

    let mut multiply_total = 0;
    let mut normal_total = 0;

    for (idx, digit) in cc.char_indices() {
        let digit = digit.to_digit(10).unwrap();
        let doubled = digit * 2;
        let d1: u32 = doubled / 10;
        let d2: u32 = doubled % 10;

        if is_cc_odd {
            if check_odd(idx as u32) {
                multiply_total += d1 + d2;
                continue;
            }
        } else {
            if !check_odd(idx as u32) {
                multiply_total += d1 + d2;
                continue;
            }
        }
        normal_total += digit;
    }

    let total = multiply_total + normal_total;

    println!("{} {}", multiply_total, normal_total);
    return total % 10 == 0;
}

fn main() {
    let cc_number = std::env::args()
        .nth(1)
        .expect("Expected a credit card number!");

    if validate_cc(&cc_number) {
        let cc_code = if cc_number.starts_with("4") || cc_number.starts_with("6") {
            &cc_number[..1]
        } else {
            &cc_number[..2]
        };
        match cc_code {
            "51" | "52" | "53" | "54" | "55" => println!("MASTERCARD"),
            "34" | "37" => println!("AMEX"),
            "35" => println!("JCB"),
            "30" => println!("Diners Club"),
            "6" => println!("DISCOVER"),
            "4" => println!("VISA"),
            _ => println!("OTHER CARD"),
        }
    } else {
        println!("INVALID");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_odd() {
        assert_eq!(check_odd(12342), false);
        assert_eq!(check_odd(9999), true);
    }

    #[test]
    fn test_validate_cc() {
        assert_eq!(validate_cc("6176292929"), false);
        assert_eq!(validate_cc("4003600000000014"), true);
        assert_eq!(validate_cc("371449635398431"), true);
        assert_eq!(validate_cc("4222222222222"), true);
    }
}
