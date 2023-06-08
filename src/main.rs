use std::string::String;
use std::io;
use std::str::FromStr;
use std::option::Option;
use std::result::Result;

fn main() {
    let number: usize;
    let radix: usize;
    println!("Enter the number in base 10 to be converted.");
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            input.clear();
            println!("Read error. Try again.");
            continue;
        }
        if let Result::Ok(parsed) = usize::from_str(input.trim()) {
            number = parsed;
            break;
        }
        input.clear();
        println!("The number must be a non negative integer. Try again.");
    }
    println!("Enter the radix of the converted number.");
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            input.clear();
            println!("Read error. Try again.");
            continue;
        }
        if let Result::Ok(parsed) = usize::from_str(input.trim()) {
            if parsed > 1 {
                radix = parsed;
                break;
            }
        }
        input.clear();
        println!("The radix must be an integer greater than 1. Try again.");
    }
    let converted = num_to_string(number, radix).unwrap(); 
    println!("{} in base {} is {}", number, radix, converted);
}

const NUM_CHARS: &str = "0123456789abcdefghijklmnopqrtsuvwxyz";

fn num_to_string(mut num: usize, radix: usize) -> Option<String> {
    if num == 0 {
        return Option::Some(String::from(NUM_CHARS.chars().next().unwrap()));
    }
    if radix < 2 || radix > NUM_CHARS.len() {
        return Option::None;
    }
    let mut string = String::new();
    while num > 0 {
        let digit = num % radix;
        num = (num - digit)/radix;
        let char = NUM_CHARS.chars().nth(digit).unwrap();
        string.insert(0, char);
    }
    Option::Some(string)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_to_string_40_2_should_return_101000() {
        let expected_result = Option::Some(String::from("101000"));

        let actual_result = num_to_string(40, 2);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn num_to_string_1_100_should_return_none() {
        let expected_result = Option::<String>::None;

        let actual_result = num_to_string(1, 100);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn num_to_string_255_16_should_return_ff() {
        let expected_result = Option::Some(String::from("ff"));

        let actual_result = num_to_string(255, 16);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn num_to_string_10_1_should_return_none() {
        let expected_result = Option::<String>::None;

        let actual_result = num_to_string(10, 1);

        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn num_to_string_10_8_should_return_12() {
        let expected_result = Option::<String>::Some(String::from("12"));

        let actual_result = num_to_string(10, 8);

        assert_eq!(actual_result, expected_result);
    }
}
