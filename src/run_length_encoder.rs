use std::{str::Chars};

pub fn encode(decoded: &str) -> String {

    if decoded.is_empty(){
        return String::from("");
    }

    if decoded.len() == 1 {
        return String::from(format!("1{}", decoded));
    }

    let mut streak_count: u32 = 1;
    let chars: Chars<'_> = decoded.chars();
    let mut initilised = false;
    let mut last_char = 'a';
    let mut result = String::from("");

    for c in chars{
        if !initilised {
            initilised = true;
            last_char = c;
        }else{
            if c == last_char{
                streak_count+=1;
            }else{
                result.push_str(format!("{}{}",streak_count, last_char).as_str());
                streak_count=1;
                last_char = c;
            }
        }
    }
    result.push_str(format!("{}{}",streak_count, last_char).as_str());
    return result;
}

fn decode(encoded: &str) -> String {
    if encoded.len() == 0{
        return String::from("");
    }
    if encoded.len()%2 == 1{
        panic!("Not a valid encoded string with odd length");
    }
    let chars = encoded.chars();
    let mut flip: bool = false;
    let mut repeat:u8 = 1;
    let mut _char = 'a';
    let mut result = String::from("");
    for c in chars{
        if !flip {
            flip = true;
            let digit: Option<u32> = c.to_digit(10);
            if (digit.is_none()){
                panic!("Expected digit");
            }
            repeat = digit.unwrap() as u8;
        }else{
            flip = false;
            if !c.is_ascii_alphabetic() {
                panic!("Not valid character");
            }
            (0..repeat).for_each(|_f| result.push(c))
        }
    }
    return result;
}


#[cfg(test)]
mod unit_tests{
    use crate::run_length_encoder::{encode, decode};


    #[test]
    pub fn should_encode(){
        assert_eq!("2a3b", encode("aabbb"));
        assert_eq!("5a", encode("aaaaa"));
        assert_eq!("", encode(""));
    }

    #[test]
    pub fn should_decode(){
        assert_eq!("aaaaBBBccc", decode("4a3B3c"));
    }

    #[test]
    #[should_panic (expected= "Not a valid encoded string with odd length")]
    pub fn should_report_odd_length_code_message(){
        decode("3a4");
    }
    
    #[test]
    #[should_panic (expected= "Not valid character")]
    pub fn should_report_malformed_message_expect_char(){
        decode("3a43");
    }
    
    #[test]
    #[should_panic (expected= "Expected digit")]
    pub fn should_report_malformed_message_expect_digit(){
        decode("3abb");
    }
}