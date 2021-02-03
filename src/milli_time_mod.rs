//! milli_time_mod

use chrono::Timelike;
use std::fmt;

/// milli_time format is fixed and global: from `000md` to `999md`, always 3 digits, no space, suffix `md`
pub struct MilliTime {
    milliday: u32,
}

impl MilliTime {
    /// constructor from hour, minute, second, rounded to 1md
    /// returns None on error
    /// # Example
    /// ```
    /// let millis = veeks_millis::MilliTime::from_hms(13,53,58).unwrap();
    /// assert_eq!(millis.to_string(),"579md");
    /// ```
    pub fn from_hms(hour: u32, minute: u32, second: u32) -> Option<Self> {
        if hour >= 24 || minute >= 60 || second >= 60 {
            return None;
        }
        //return
        Some(MilliTime {
            milliday: ((hour * 3600 + minute * 60 + second) as f64 / 86.4).round() as u32,
        })
    }

    /// constructor from chrono NaiveTime, rounded to 1md
    /// # Example
    ///
    /// ```
    /// let nt = chrono::NaiveTime::from_hms(13,30,00);
    /// let millis = veeks_millis::MilliTime::from_naive_time(nt);
    /// assert_eq!(millis.milliday(),563);
    /// ```
    pub fn from_naive_time(nt: chrono::NaiveTime) -> Self {
        // return
        MilliTime {
            milliday: (nt.num_seconds_from_midnight() as f64 / 86.4).round() as u32,
        }
    }
    /// constructor from string millis `000md` to `999md`
    /// returns None if unrecognized format
    /// # Example
    ///
    /// ```
    /// let millis = veeks_millis::MilliTime::from_str("560md").unwrap();
    /// assert_eq!(millis.milliday(),560);
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        // ok() Converts from Result<T, E> to Option<T>.
        if s.len() != 5 {
            return None;
        };
        let milliday = s.get(0..3)?.parse::<u32>().ok()?;
        let millis_unit = s.get(3..5)?;
        if millis_unit != "md" {
            return None;
        };
        // return
        Some(MilliTime { milliday })
    }
    /// getter milliday f64
    /// # Example
    /// ```
    /// let veeks = veeks_millis::MilliTime::from_hms(13,53,58).unwrap();
    /// assert_eq!(veeks.milliday(),579);
    /// ```
    pub fn milliday(&self) -> u32 {
        // return
        self.milliday
    }

    /// convert millis to chrono NaiveTime rounded to 1 second
    /// returns None if error
    /// # Example
    ///
    /// ```
    /// let millis = veeks_millis::MilliTime::from_str("560md").unwrap();
    /// let nt = millis.to_naive_time().unwrap();
    /// assert_eq!(nt,chrono::NaiveTime::from_hms(13,26,24));
    /// ```
    pub fn to_naive_time(&self) -> Option<chrono::NaiveTime> {
        chrono::NaiveTime::from_num_seconds_from_midnight_opt(
            (self.milliday as f64 * 86.4).round() as u32,
            0,
        )
    }
}

/// the Trait Display implements fn to_string() implicitly
impl fmt::Display for MilliTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // return
        write!(f, r#"{:03}md"#, self.milliday)
    }
}

#[cfg(test)]
mod test {
    // use unwrap::unwrap;

    #[test]
    pub fn t07_naive_time_to_millis() {
        let nt = chrono::NaiveTime::from_hms(13, 30, 00);
        let millis = super::MilliTime::from_naive_time(nt);
        assert_eq!(millis.milliday, 563);
    }

    #[test]
    pub fn t08_naive_time_to_millis_str() {
        let nt = chrono::NaiveTime::from_hms(13, 30, 00);
        let millis = super::MilliTime::from_naive_time(nt);
        assert_eq!(millis.to_string(), "563md");
    }

    #[test]
    pub fn t09_millis_from_str_opt() {
        let millis = super::MilliTime::from_str("560md").unwrap();
        assert_eq!(millis.milliday(), 560);
    }

    #[test]
    #[should_panic]
    pub fn t10_millis_from_str_opt_should_panic() {
        let _millis = super::MilliTime::from_str("560 md").unwrap();
    }

    #[test]
    pub fn t11_millis_to_naive_time_opt() {
        let millis = super::MilliTime::from_str("560md").unwrap();
        let nt = millis.to_naive_time().unwrap();
        assert_eq!(nt, chrono::NaiveTime::from_hms(13, 26, 24));
    }
}
