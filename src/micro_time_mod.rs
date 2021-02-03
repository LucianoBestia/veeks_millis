//! micro_time_mod

use std::fmt;

/// micro_time format with decimals: `12.34μd`, decimal digits, no space, suffix `μd`
pub struct MicroTime {
    microday: f64,
}

impl MicroTime {
    /// constructor from microdays f64
    /// returns None on error
    /// # Example
    /// ```
    /// let micros = veeks_millis::MicroTime::from_f64(110.9);
    /// assert_eq!(micros.microday(),110.9);
    /// ```
    pub fn from_f64(microday: f64) -> Self {
        //return
        MicroTime { microday: microday }
    }
    /// constructor from seconds
    /// returns None on error
    /// # Example
    /// ```
    /// let micros = veeks_millis::MicroTime::from_seconds(9.58);
    /// assert_eq!(micros.microday(),110.87962962962962);
    /// ```
    pub fn from_seconds(seconds: f64) -> Self {
        //return
        MicroTime {
            microday: seconds / 0.0864,
        }
    }
    /// constructor from string micros
    /// returns None if unrecognized format
    /// the string format is fixed:a decimal number followed by "μd". No space.
    /// # Example
    ///
    /// ```
    /// let micros = veeks_millis::MicroTime::from_str("110.8μd").unwrap();
    /// assert_eq!(micros.microday(),110.8);
    /// ```
    pub fn from_str(micros: &str) -> Option<MicroTime> {
        // ok() Converts from Result<T, E> to Option<T>.
        let micros_unit = micros.get(micros.len() - 3..micros.len())?;
        if micros_unit != "μd" {
            return None;
        };
        let microday = micros.get(0..micros.len() - 3)?.parse::<f64>().ok()?;
        // return
        Some(MicroTime { microday })
    }

    /// convert micros (microdays) f64 to seconds f64
    /// # Example
    ///
    /// ```
    /// use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};
    /// let micros = veeks_millis::MicroTime::from_str("110.9μd").unwrap();
    /// let seconds = micros.to_seconds();
    /// assert_eq!(seconds,9.581760000000001);
    /// ```
    pub fn to_seconds(&self) -> f64 {
        self.microday() * 0.0864
    }
    /// getter microday f64
    /// # Example
    /// ```
    /// let micros = veeks_millis::MicroTime::from_f64(12.34);
    /// assert_eq!(micros.microday(),12.34);
    /// ```
    pub fn microday(&self) -> f64 {
        // return
        self.microday
    }
}

/// the Trait Display implements fn to_string() implicitly
/// round to 3 decimal places
impl fmt::Display for MicroTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // return
        write!(f, r#"{:.3}μd"#, self.microday)
    }
}

#[cfg(test)]
mod test {
    // use unwrap::unwrap;

    #[test]
    pub fn t12_seconds_to_micros() {
        let micros = super::MicroTime::from_seconds(9.58);
        assert_eq!(micros.microday(), 110.87962962962962);
    }

    #[test]
    pub fn t13_micros_from_str_opt() {
        let micros = super::MicroTime::from_str("110.8μd").unwrap();
        assert_eq!(micros.microday(), 110.8);
    }

    #[test]
    pub fn t14_micros_to_seconds() {
        let micros = super::MicroTime::from_f64(110.0);
        let seconds = micros.to_seconds();
        assert_eq!(seconds, 9.504000000000001);
    }
    #[test]
    pub fn t15_micros_to_str() {
        let micros = super::MicroTime::from_f64(110.0);
        assert_eq!(micros.to_string(), "110.000μd");
    }
}
