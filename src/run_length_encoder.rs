use std::{str::Chars};

pub fn encode(decoded: &str) -> String {


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

#[cfg(test)]
mod unit_tests{
    use crate::run_length_encoder::encode;


    #[test]
    pub fn should_encode(){
        assert_eq!("2a3b", encode("aabbb"));
        assert_eq!("5a", encode("aaaaa"))
    }

}