use std::cmp::Ordering;


enum Temperature_scale{
    Farenheit,
    Celsius
}


pub struct Temperature {
    magnitude: f32,
    scale: Temperature_scale
}

impl Ord for Temperature {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       let _self_c = self.convert_to_c();
       let _other_c = other.convert_to_c();
       if _self_c < _other_c {
            return Ordering::Less
       }else if _self_c > _other_c {
            return Ordering::Less
       }
       Ordering::Equal
    }
}

impl Eq for Temperature {
    
}

impl PartialOrd for Temperature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.convert_to_c().partial_cmp(&other.convert_to_c())
    }
}

impl PartialEq for Temperature {
    fn eq(&self, other: &Self) -> bool {
        self.convert_to_c().eq(&other.convert_to_c())
    }
}

impl Temperature{
    pub fn convert_to_c(&self) -> f32{
        match self.scale{
            Temperature_scale::Farenheit => ((self.magnitude - 32.0 ) / 9.0 ) * 5.0,
            Temperature_scale::Celsius => self.magnitude
        }
    }
    
    pub fn convert_to_f(&self) -> f32{
        match self.scale{
            Temperature_scale::Farenheit => self.magnitude,
            Temperature_scale::Celsius => self.magnitude * 1.8 + 32.0
        }
    }
}

pub struct Weather{
    temp: Temperature
}

impl Weather {
    pub fn warmer(&self, other: &Self) -> bool {
        return self.temp > other.temp;
    }
}

#[cfg(test)]
pub mod temperature_conversions{

    use super::{Temperature_scale, Temperature, Weather};


    #[test]
    pub fn should_convert_c_to_f(){
        let _32c = Temperature{magnitude:32.0, scale:Temperature_scale::Celsius};
        let _40f = Temperature{magnitude:40.0, scale:Temperature_scale::Farenheit};
        assert_eq!(_32c.convert_to_c(), 32.0);
        assert_eq!(format!("{:.2}",_40f.convert_to_c()),"4.44");
        assert_eq!(format!("{:.2}",_32c.convert_to_f()),"89.60");
        assert_eq!(format!("{:.2}",_40f.convert_to_f()),"40.00");
    }

    #[test]
    pub fn should_identify_warmer_weather(){
        let w_32c = Weather{
            temp: Temperature { magnitude: 32.0, scale: Temperature_scale::Celsius }
        };
        
        let w_33c = Weather{
            temp: Temperature { magnitude: 33.0, scale: Temperature_scale::Celsius }
        };

        assert!(!w_32c.warmer(&w_33c));
        assert!(w_33c.warmer(&w_32c));
    }

    #[test]
    pub fn sort_temperatures(){
        let mut temps = [Temperature{magnitude:10.0, scale:Temperature_scale::Celsius},
        Temperature{magnitude:11.0, scale:Temperature_scale::Farenheit},
        Temperature{magnitude:12.0, scale:Temperature_scale::Celsius}];
        temps.sort();
        assert_eq!(temps[0].magnitude, 11.0);
        assert_eq!(temps[1].magnitude, 10.0);
        assert_eq!(temps[2].magnitude, 12.0);
    }

}

