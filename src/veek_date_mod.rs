//! veek_date_mod

use chrono::Datelike;
use std::fmt;
/// veek_date format is fixed and global:  
/// 4 digit year has the unit c for CE common era, one space  
/// 2 digit for veek (very similar to week) from 01v to 53v, has unit v, one space  
/// 1 digit for veek-day from 1d (monday) to 7d(sunday)    
/// example: `2021c 53v 7d`  
pub struct VeekDate {
    year: u32,
    veek: u32,
    day: u32,
}

impl VeekDate {
    /// constructor from year, veek, day
    /// returns None on error
    /// # Example
    /// ```
    /// let veeks = veeks_millis::VeekDate::from_yvd(2021,09,3).unwrap();
    /// assert_eq!(veeks.to_string(),"2021c 09v 3d");
    /// ```
    pub fn from_yvd(year: u32, veek: u32, day: u32) -> Option<Self> {
        if year < 1000 || year > 9999 || veek < 1 || veek > 53 || day < 1 || day > 7 {
            return None;
        }
        //return
        Some(VeekDate { year, veek, day })
    }

    /// constructor from year, ordinal_day
    /// returns None on error
    /// # Example
    /// ```
    /// let veeks = veeks_millis::VeekDate::from_year_ordinal_day(2021,45).unwrap();
    /// assert_eq!(veeks.to_string(),"2021c 07v 3d");
    /// ```
    pub fn from_year_ordinal_day(year: u32, ordinal_day: u32) -> Option<Self> {
        let veek = ((ordinal_day as f64 - 0.1) / 7.0).floor() as u32 + 1;
        let day = ((ordinal_day as f64 - 0.1) % 7.0).round() as u32;
        // return
        Self::from_yvd(year, veek, day)
    }

    /// constructor from NaiveDate
    /// # Example
    ///
    /// ```
    /// let nd = chrono::NaiveDate::from_ymd(2021,02,28);
    /// let veeks = veeks_millis::VeekDate::from_naive_date(nd).unwrap();
    /// assert_eq!(veeks.to_string() ,"2021c 09v 3d");
    /// ```
    pub fn from_naive_date(nd: chrono::NaiveDate) -> Option<Self> {
        // return
        Self::from_year_ordinal_day(nd.year() as u32, nd.ordinal() as u32)
    }

    /// constructor from year, month, day
    /// returns None on error
    /// # Example
    /// ```
    /// let veeks = veeks_millis::VeekDate::from_ymd(2021,02,28).unwrap();
    /// assert_eq!(veeks.to_string(),"2021c 09v 3d");
    /// ```
    pub fn from_ymd(year: u32, month: u32, day: u32) -> Option<Self> {
        match chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32) {
            // return
            Some(nd) => Self::from_naive_date(nd),
            None => None,
        }
    }

    /// constructor from str in VeekDate format
    /// returns None on error
    /// # Example
    /// ```
    /// let veeks = veeks_millis::VeekDate::from_str("2021c 09v 3d").unwrap();
    /// assert_eq!(veeks.to_string(),"2021c 09v 3d");
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        // ok() Converts from Result<T, E> to Option<T>.
        if s.len() != 12 {
            return None;
        };
        let year = s.get(0..4)?.parse::<u32>().ok()?;
        let year_unit = s.get(4..6)?;
        if year_unit != "c " {
            return None;
        };
        let veek = s.get(6..8)?.parse::<u32>().ok()?;
        let veek_unit = s.get(8..10)?;
        if veek_unit != "v " {
            return None;
        };
        let day = s.get(10..11)?.parse::<u32>().ok()?;
        let day_unit = s.get(11..12)?;
        if day_unit != "d" {
            return None;
        };
        // return
        Self::from_yvd(year, veek, day)
    }

    /// getter year u32
    /// # Example
    /// ```
    /// let veeks = veeks_millis::VeekDate::from_str("2021c 09v 3d").unwrap();
    /// assert_eq!(veeks.year(),2021);
    /// ```
    pub fn year(&self) -> u32 {
        // return
        self.year
    }
    /// getter veek u32
    /// # Example
    /// ```
    /// let veeks = veeks_millis::VeekDate::from_str("2021c 09v 3d").unwrap();
    /// assert_eq!(veeks.veek(),9);
    /// ```
    pub fn veek(&self) -> u32 {
        // return
        self.veek
    }

    /// getter day u32
    /// # Example
    /// ```
    /// let veeks = veeks_millis::VeekDate::from_str("2021c 09v 3d").unwrap();
    /// assert_eq!(veeks.day(),3);
    /// ```
    pub fn day(&self) -> u32 {
        // return
        self.day
    }

    /// convert veek_date to chrono NaiveDate
    /// returns None on error
    /// # Example
    ///
    /// ```
    /// let vd = veeks_millis::VeekDate::from_ymd(2021,02,28).unwrap();
    /// let nd = vd.to_naive_date().unwrap();
    /// assert_eq!(nd, chrono::NaiveDate::from_ymd(2021,02,28));
    /// ```
    pub fn to_naive_date(&self) -> Option<chrono::NaiveDate> {
        // return
        chrono::NaiveDate::from_yo_opt(self.year as i32, ((self.veek - 1) * 7 + self.day) as u32)
    }
}

/// the Trait Display implements fn to_string() implicitly
impl fmt::Display for VeekDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // return
        write!(f, r#"{:04}c {:02}v {:01}d"#, self.year, self.veek, self.day)
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;
    // use unwrap::unwrap;

    #[test]
    pub fn t01_naive_date_to_veek_date() {
        let nd = NaiveDate::from_ymd(2021, 02, 28);
        let veeks = super::VeekDate::from_naive_date(nd).unwrap();
        assert_eq!(veeks.to_string(), "2021c 09v 3d");
    }
    #[test]
    pub fn t02_naive_date_to_veek_date() {
        let nd = NaiveDate::from_ymd(2021, 05, 01);
        let veeks = super::VeekDate::from_naive_date(nd).unwrap();
        assert_eq!(veeks.to_string(), "2021c 18v 2d");
    }
    #[test]
    pub fn t03_naive_date_to_veek_date() {
        let nd = NaiveDate::from_ymd(2021, 12, 25);
        let veeks = super::VeekDate::from_naive_date(nd).unwrap();
        assert_eq!(veeks.to_string(), "2021c 52v 2d");
    }
    #[test]
    pub fn t04_veek_to_naive_date_opt() {
        let veek = super::VeekDate::from_str("2021c 09v 3d").unwrap();
        let nd = veek.to_naive_date().unwrap();
        assert_eq!(nd, NaiveDate::from_ymd(2021, 02, 28));
    }
    #[test]
    pub fn t05_veek_to_naive_date_opt() {
        let veek = super::VeekDate::from_str("2021c 52v 2d").unwrap();
        let nd = veek.to_naive_date().unwrap();
        assert_eq!(nd, NaiveDate::from_ymd(2021, 12, 25));
    }
    #[test]
    #[should_panic]
    pub fn t06_veek_to_naive_date_opt_should_panic() {
        // panic because the veek format is wrong
        let _veek = super::VeekDate::from_str("2021c-09v-3d").unwrap();
    }
}
