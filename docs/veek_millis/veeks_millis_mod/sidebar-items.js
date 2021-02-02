initSidebarItems({"fn":[["micros_from_str_opt","convert string micros to f64 micros (microdays) returns None if unrecognized format"],["micros_to_seconds","convert micros (microdays) f64 to seconds f64"],["millis_from_str_opt","convert string millis into f64 millis returns None if unrecognized format"],["millis_to_naive_time_opt","convert f64 millis to chrono NaiveTime returns None if error"],["naive_date_to_veek_date","convert chrono NaiveDate into String with veek-date format veek-date format in short: 4 digit year has the unit c for CE common era, one space 2 digit for veek (very similar to week) from 01v to 53v, has unit v, one space 1 digit for veek-day from 1d (monday) to 7d(sunday)"],["naive_time_to_millis","convert chrono NaiveTime into float millis be careful that floats must be rounded before presenting it to humans"],["naive_time_to_millis_str","convert chrono NaiveTime into String with millis format rounded to 1 milliday"],["seconds_to_micros","convert seconds to micros (microdays)"],["veek_to_naive_date_opt","convert String with veek format to chrono NaiveDate return None if the string is not in a correct veek-format veek format in short: 4 digit year has the unit c for CE common era, one space 2 digit for veek (very similar to week) from 01v to 53v, has unit v, one space 1 digit for veek-day from 1d (monday) to 7d(sunday)"]]});