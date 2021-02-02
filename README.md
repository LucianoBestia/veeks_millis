[comment]: # (lmake_md_to_doc_comments segment start A)

# veek_millis

[comment]: # (lmake_cargo_toml_to_md start)

**2021 new date time units and formats: veeks and millis**  
***[repo](https://github.com/LucianoBestia/veek_millis); version: 0.5.4  date: 2021-02-02 authors: Luciano Bestia***  

[comment]: # (lmake_cargo_toml_to_md end)

 [![crates.io](https://meritbadge.herokuapp.com/veek_millis)](https://crates.io/crates/veek_millis) [![Documentation](https://docs.rs/veek_millis/badge.svg)](https://docs.rs/veek_millis/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/veek_millis.svg)](https://web.crev.dev/rust-reviews/crate/veek_millis/) [![RustActions](https://github.com/LucianoBestia/veek_millis/workflows/rust/badge.svg)](https://github.com/LucianoBestia/veek_millis/) [![latest doc](https://img.shields.io/badge/latest_docs-GitHub-orange.svg)](https://lucianobestia.github.io/veek_millis/veek_millis/index.html) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/veek_millis/blob/master/LICENSE)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-79-green.svg)]()
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-142-blue.svg)]()
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-11-purple.svg)]()
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)]()
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-88-orange.svg)]()

[comment]: # (lmake_lines_of_code end)

## Proposal for a new date and time units and formats

Veeks and millis are my suggestion for new date-time units and formats.  
Here is a long read about the date-time reform proposal:  
<https://github.com/LucianoBestia/new_date_time_units_and_formats/>

A super short explanation:  
Months are obsolete and they are not used in the new format.  
Years remain the same.  
Veek is similar to week. One year has 52 full veeks with 7 days.  
The exception is the last 53rd veek that has only 1 celebration day. For leap years there are 2 celebration days.  
The new year always starts with `01v 1d` - short pronunciation one-vee one-dee. Basically every year starts with a monday.  
Veek-days are similar to week-days, but without names. They use just numbers and the `d` unit.  
`1d` is the new name for Monday and `7d` is the new name for Sunday.  
The veek-date format is global for every language and looks exactly like this: `2021c 52v 2d`  
The year must have 4 digits from 1000c to 9999c. The unit `c` stands for CE - common era. Space.  
The veek must have 2 digits from 01v to 53v. The unit `v` stands for veek. Space.  
The day must have 1 digit from 1d to 7d. The unit `d` stands for day. End.  

Hours, minutes and seconds are obsolete.  
One day is divided into 1000md.  
Millis is the short name for milliday. The unit is `md`.  
For shorter time intervals there is microdays or micros, unit `μd`. `1md` has `1000μd`.  

This crate contains functions to use with the new units veeks and millis.  
It is dependent on the crate `chrono` for `NaiveDate` and `NaiveTime`.  

## Used in projects

<https://github.com/LucianoBestia/new_date_time_units_and_formats/>  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

[comment]: # (lmake_md_to_doc_comments segment end A)
