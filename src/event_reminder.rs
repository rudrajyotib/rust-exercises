

use chrono::{NaiveDate, Local, Datelike};

pub struct ImportantEvent<'a>{
    pub event_name: &'a str,
    event_date: NaiveDate
}

pub struct Diary<'a > {
    events : Vec<ImportantEvent<'a>>
}

trait DeadLine{
    fn is_passed(& self) -> bool;
}

impl<'a> Diary<'a> {
    pub fn past_events(&self) -> Vec<&ImportantEvent>{
        let mut past_events = Vec::new();
        for event in &self.events{
            if event.is_passed() {
                past_events.push(event);
            }
        }
        return past_events;
    }

    pub fn upcoming_events(&self) -> Vec<&ImportantEvent> {
        let mut upcoming_events = Vec::new();
        for event in &self.events{
            if !event.is_passed() {
                upcoming_events.push(event);
            }
        }
        return upcoming_events;
    }

    pub fn new()->Diary<'static>{
        let diary = Diary{
            events: Vec::new()
        };
        return diary
    }

    pub fn add_event(& mut self, event_name:&'a String, event_date : &NaiveDate){
            self.events.push(ImportantEvent { 
             event_name: event_name,
             event_date: NaiveDate::from_ymd_opt(event_date.year(), event_date.month(), event_date.day()).unwrap() 
            });
    }
}

impl DeadLine for ImportantEvent<'_>{
    fn is_passed(&self)->bool{
        let time_now = Local::now();
        self.event_date < NaiveDate::from_ymd_opt(time_now.year(), time_now.month(), time_now.day()).unwrap()
    }
}

#[cfg(test)]
pub mod unit_tests_events
{
    use chrono::{NaiveDate, Local, Datelike};

    use crate::event_reminder::DeadLine;

    use super::ImportantEvent;


    #[test]
    pub fn should_identify_past(){
        let time_now = Local::now();
        let imp = ImportantEvent{
            event_name : "abc",
            event_date : NaiveDate::from_ymd_opt(time_now.year()-2, 1, 31).unwrap()
        };

        assert!(imp.is_passed())
    }
    
    #[test]
    pub fn should_identify_future(){
        let time_now = Local::now();
        let imp = ImportantEvent{
            event_name : "abc",
            event_date : NaiveDate::from_ymd_opt(time_now.year()+2, 1, 31).unwrap()
        };

        assert!(!imp.is_passed())
    }

}


#[cfg(test)]
mod unit_tests_diary{
    use chrono::{Local, Datelike, NaiveDate};

    use super::Diary;


    #[test]
    pub fn should_operate_on_diary(){
        let mut diary = Diary::new();
        let time_now = Local::now();
        let passed_events = [String::from("event1"), String::from("event2")];
        let future_events = [String::from("event3"), String::from("event4")];
        diary.add_event(&passed_events[0],  &NaiveDate::from_ymd_opt(time_now.year()-2, 1, 31).unwrap());
        diary.add_event(&passed_events[1],  &NaiveDate::from_ymd_opt(time_now.year()-2, 1, 31).unwrap());
        diary.add_event(&future_events[0],  &NaiveDate::from_ymd_opt(time_now.year()+2, 1, 31).unwrap());
        diary.add_event(&future_events[1],  &NaiveDate::from_ymd_opt(time_now.year()+2, 1, 31).unwrap());
        let past_event_names = diary.past_events().iter().map(|f| f.event_name).collect::<Vec<&str>>();
        let future_event_names = diary.upcoming_events().iter().map(|f| f.event_name).collect::<Vec<&str>>();
        assert!(past_event_names.contains(&passed_events[0].as_str()));
        assert!(past_event_names.contains(&passed_events[1].as_str()));
        assert!(future_event_names.contains(&future_events[0].as_str()));
        assert!(future_event_names.contains(&future_events[1].as_str()));
    }


}