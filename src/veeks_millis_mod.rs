//! veek_millis_mod

use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};

/// convert chrono NaiveDate into String with veek-date format
/// veek-date format in short:
/// 4 digit year has the unit c for CE common era, one space
/// 2 digit for veek (very similar to week) from 01v to 53v, has unit v, one space
/// 1 digit for veek-day from 1d (monday) to 7d(sunday)  
/// # Example
///
/// ```
/// use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};
/// let nd = NaiveDate::from_ymd(2021,02,28);
/// let veeks = veek_millis::naive_date_to_veek_date(nd);
/// assert_eq!(veeks,"2021c 09v 3d");
/// ```
pub fn naive_date_to_veek_date(nd: NaiveDate) -> String {
    // return
    format!(
        r#"{}c {:02}v {}d"#,
        nd.year(),
        ((nd.ordinal() as f64 - 0.1) / 7.0).floor() as u32 + 1,
        ((nd.ordinal() as f64 - 0.1) % 7.0).round() as u32
    )
}

/// convert String with veek format to chrono NaiveDate
/// return None if the string is not in a correct veek-format
/// veek format in short:
/// 4 digit year has the unit c for CE common era, one space
/// 2 digit for veek (very similar to week) from 01v to 53v, has unit v, one space
/// 1 digit for veek-day from 1d (monday) to 7d(sunday)  
/// # Example
///
/// ```
/// use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};
/// let nd = veek_millis::veek_to_naive_date_opt("2021c 09v 3d").unwrap();
/// assert_eq!(nd, NaiveDate::from_ymd(2021,02,28));
/// ```
pub fn veek_to_naive_date_opt(s: &str) -> Option<NaiveDate> {
    // the format is fixed with space after c and v
    use regex::Regex;
    let re = Regex::new(r"^\d{4}c [0-5][0-9]v [1-7]d$").unwrap();
    use std::str::FromStr;
    if re.is_match(s) {
        NaiveDate::from_yo_opt(
            i32::from_str(&s[0..4]).unwrap_or(0),
            (u32::from_str(&s[6..8]).unwrap_or(0) - 1) * 7 + u32::from_str(&s[10..11]).unwrap_or(0),
        )
    } else {
        return None;
    }
}

/// convert chrono NaiveTime into float millis
/// be careful that floats must be rounded before presenting it to humans
/// # Example
///
/// ```
/// use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};
/// let nt = NaiveTime::from_hms(13,30,00);
/// let millis = veek_millis::naive_time_to_millis(nt);
/// assert_eq!(millis,562.5);
/// ```
pub fn naive_time_to_millis(nt: NaiveTime) -> f64 {
    // return
    nt.num_seconds_from_midnight() as f64 / 86.4
}

/// convert chrono NaiveTime into String with millis format
/// rounded to 1 milliday
/// # Example
///
/// ```
/// use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};
/// let nt = NaiveTime::from_hms(13,30,00);
/// let millis = veek_millis::naive_time_to_millis_str(nt);
/// assert_eq!(millis,"563md");
/// ```
pub fn naive_time_to_millis_str(nt: NaiveTime) -> String {
    // return
    format!(r#"{}md"#, naive_time_to_millis(nt).round())
}

/// convert string millis into f64 millis
/// returns None if unrecognized format
/// # Example
///
/// ```
/// let millis = veek_millis::millis_from_str_opt("560md").unwrap();
/// assert_eq!(millis,560.0);
/// ```
pub fn millis_from_str_opt(millis: &str) -> Option<f64> {
    // the format is fixed:a decimal number followed by "md" no space
    use regex::Regex;
    let re = Regex::new(r"^\d*(\.\d+)?md$").unwrap();
    use std::str::FromStr;
    if re.is_match(millis) {
        let millis = millis.strip_suffix("md").unwrap();
        let millis = f64::from_str(millis);
        // return
        match millis {
            Ok(millis) => Some(millis),
            Err(_err) => None,
        }
    } else {
        return None;
    }
}

/// convert f64 millis to chrono NaiveTime
/// returns None if error
/// # Example
///
/// ```
/// use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};
/// let millis = veek_millis::millis_from_str_opt("560md").unwrap();
/// let nt = veek_millis::millis_to_naive_time_opt(millis).unwrap();
/// assert_eq!(nt,NaiveTime::from_hms(13,26,24));
/// ```
pub fn millis_to_naive_time_opt(millis: f64) -> Option<NaiveTime> {
    NaiveTime::from_num_seconds_from_midnight_opt((millis * 86.4).round() as u32, 0)
}

/// convert seconds to micros (microdays)
/// # Example
///
/// ```
/// use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};
/// let micros = veek_millis::seconds_to_micros(9.58);
/// assert_eq!(micros,110.87962962962962);
/// ```
pub fn seconds_to_micros(seconds: f64) -> f64 {
    seconds / 0.0864
}

/// convert string micros to f64 micros (microdays)
/// returns None if unrecognized format
/// # Example
///
/// ```
/// use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};
/// let micros = veek_millis::micros_from_str_opt("110μd").unwrap();
/// assert_eq!(micros,110.0);
/// ```
pub fn micros_from_str_opt(micros: &str) -> Option<f64> {
    // the string format is fixed:a decimal number followed by "μd". No space.
    use regex::Regex;
    let re = Regex::new(r"^\d*(\.\d+)?μd$").unwrap();
    use std::str::FromStr;
    if re.is_match(micros) {
        let micros = micros.strip_suffix("μd").unwrap();
        let micros = f64::from_str(micros).unwrap_or(0.0);
        return Some(micros);
    } else {
        return None;
    }
}

/// convert micros (microdays) f64 to seconds f64
/// # Example
///
/// ```
/// use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};
/// let seconds = veek_millis::micros_to_seconds(110.9);
/// assert_eq!(seconds,9.581760000000001);
/// ```
pub fn micros_to_seconds(micros: f64) -> f64 {
    micros * 0.0864
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveTime};
    // use unwrap::unwrap;

    #[test]
    pub fn t01_naive_date_to_veek_date() {
        let nd = NaiveDate::from_ymd(2021, 02, 28);
        let veeks = super::naive_date_to_veek_date(nd);
        assert_eq!(veeks, "2021c 09v 3d");
    }
    #[test]
    pub fn t02_naive_date_to_veek_date() {
        let nd = NaiveDate::from_ymd(2021, 05, 01);
        let veeks = super::naive_date_to_veek_date(nd);
        assert_eq!(veeks, "2021c 18v 2d");
    }
    #[test]
    pub fn t03_naive_date_to_veek_date() {
        let nd = NaiveDate::from_ymd(2021, 12, 25);
        let veeks = super::naive_date_to_veek_date(nd);
        assert_eq!(veeks, "2021c 52v 2d");
    }
    #[test]
    pub fn t04_veek_to_naive_date_opt() {
        let nd = super::veek_to_naive_date_opt("2021c 09v 3d").unwrap();
        assert_eq!(nd, NaiveDate::from_ymd(2021, 02, 28));
    }
    #[test]
    pub fn t05_veek_to_naive_date_opt() {
        let nd = super::veek_to_naive_date_opt("2021c 52v 2d").unwrap();
        assert_eq!(nd, NaiveDate::from_ymd(2021, 12, 25));
    }
    #[test]
    #[should_panic]
    pub fn t06_veek_to_naive_date_opt_should_panic() {
        // panic because the veek format is wrong
        let _nd = super::veek_to_naive_date_opt("2021c-09v-3d").unwrap();
    }

    #[test]
    pub fn t07_naive_time_to_millis() {
        let nt = NaiveTime::from_hms(13, 30, 00);
        let millis = super::naive_time_to_millis(nt);
        assert_eq!(millis, 562.5);
    }

    #[test]
    pub fn t08_naive_time_to_millis_str() {
        let nt = NaiveTime::from_hms(13, 30, 00);
        let millis = super::naive_time_to_millis_str(nt);
        assert_eq!(millis, "563md");
    }

    #[test]
    pub fn t09_millis_from_str_opt() {
        let millis = super::millis_from_str_opt("560md").unwrap();
        assert_eq!(millis, 560.0);
    }

    #[test]
    #[should_panic]
    pub fn t10_millis_from_str_opt_should_panic() {
        let _millis = super::millis_from_str_opt("560 md").unwrap();
    }

    #[test]
    pub fn t11_millis_to_naive_time_opt() {
        let millis = super::millis_from_str_opt("560md").unwrap();
        let nt = super::millis_to_naive_time_opt(millis).unwrap();
        assert_eq!(nt, NaiveTime::from_hms(13, 26, 24));
    }

    #[test]
    pub fn t12_seconds_to_micros() {
        let micros = super::seconds_to_micros(9.58);
        assert_eq!(micros, 110.87962962962962);
    }

    #[test]
    pub fn t13_micros_from_str_opt() {
        let micros = super::micros_from_str_opt("110μd").unwrap();
        assert_eq!(micros, 110.0);
    }

    #[test]
    pub fn t14_micros_to_seconds() {
        let seconds = super::micros_to_seconds(110.9);
        assert_eq!(seconds, 9.581760000000001);
    }
}
