
    use std::{collections::{HashSet}, fmt::Debug};

    pub fn median(mut arr: &mut Vec<f64>) -> Option<f64> {

        if arr.is_empty() {
            return None;
        }
        arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if arr.len() % 2 == 0 {
            return Some((arr[arr.len() / 2] + arr[(arr.len() / 2) - 1]) / 2.0);
        } else {
            return Some(arr[(arr.len() - 1) / 2]);
        }
        
    }

    pub fn filter_duplicates(mut arr: Vec<u64>) -> Vec<u64> {
        let mut hash_set: HashSet<u64> = HashSet::new();
        let mut result: Vec<u64> = vec![];
        for element in arr.iter() {
            if hash_set.contains(element) {
                continue;
            } else {
                result.push(*element);
                hash_set.insert(*element);
            }
        }
        return result;
    }

    pub fn sort_text<T: AsRef<str> + Debug>(mut arr: &mut Vec<T>){
        arr.sort_by(|a, b| a.as_ref().to_lowercase().partial_cmp(&b.as_ref().to_lowercase()).unwrap());
    }


#[cfg(test)]
pub mod unit_tests {

    pub mod median_tests {
        use crate::statistics::{median, sort_text};

        #[test]
        pub fn should_find_median_of_empty_vector() {
            let mut input = vec![];
            let opt = median(&mut input);
            assert_eq!(opt, None);
            let mut strings =  vec![String::from("ABD"),String::from("abc")];
            let mut strs = vec!["ABD","abc","ABe"];
            sort_text(&mut strings);
            sort_text(&mut strs);
            print!("Received Strings::{:?}",strings);
            print!("Received Strings::{:?}",strs);
        }

        #[test]
        pub fn should_find_median_of_odd_size_vector() {
            let mut input = vec![10.0, 8.0, 5.0, 7.0, 6.0];
            let opt = median(&mut input);
            assert_eq!(opt.unwrap(), 7.0);
        }

        #[test]
        pub fn should_find_median_of_even_size_vector() {
            let mut input = vec![10.0, 8.0, 5.0, 7.0, 6.0, 4.0];
            let opt = median(&mut input);
            print!("Vector has now become::{:?}", input);
            assert_eq!(opt.unwrap(), 6.5);
        }
    }

    pub mod unique_tests{
        use crate::statistics::filter_duplicates;

        #[test]
        pub fn should_supply_unique_in_order(){
            let arr = filter_duplicates(vec![1,1,2,3,3,4,2,]);
            assert_eq!(arr, vec![1,2,3,4])
        }   
        
         #[test]
        pub fn should_support_filtering_empty_array(){
            let arr = filter_duplicates(vec![]);
            assert_eq!(arr, vec![])
        }   
    }
}
