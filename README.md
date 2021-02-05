[comment]: # (lmake_md_to_doc_comments segment start A)

# veeks_millis

[comment]: # (lmake_cargo_toml_to_md start)

**2021 new date time units and formats: veeks and millis**  
***[repo](https://github.com/LucianoBestia/veeks_millis); version: 1.0.2  date: 2021-02-03 authors: Luciano Bestia***  

[comment]: # (lmake_cargo_toml_to_md end)

 [![crates.io](https://meritbadge.herokuapp.com/veeks_millis)](https://crates.io/crates/veeks_millis) [![Documentation](https://docs.rs/veeks_millis/badge.svg)](https://docs.rs/veeks_millis/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/veeks_millis.svg)](https://web.crev.dev/rust-reviews/crate/veeks_millis/) [![RustActions](https://github.com/LucianoBestia/veeks_millis/workflows/rust/badge.svg)](https://github.com/LucianoBestia/veeks_millis/) [![latest doc](https://img.shields.io/badge/latest_docs-GitHub-orange.svg)](https://lucianobestia.github.io/veeks_millis/veeks_millis/index.html) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/veeks_millis/blob/master/LICENSE)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-177-green.svg)](https://github.com/LucianoBestia/veeks_millis/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-206-blue.svg)](https://github.com/LucianoBestia/veeks_millis/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-29-purple.svg)](https://github.com/LucianoBestia/veeks_millis/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/veeks_millis/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-100-orange.svg)](https://github.com/LucianoBestia/veeks_millis/)

[comment]: # (lmake_lines_of_code end)

## Proposal for a new date and time units and formats

Veeks and millis are my suggestion for new date-time units and formats.  
Here is a long read TL;DR about the date-time reform proposal:  
<https://github.com/LucianoBestia/new_date_time_units_and_formats/>

A super short explanation:  
Years (beginning and end) remain the same as in the CE - common era calendar.  
Months are obsolete and they are not used in the new format.  
Veek is similar to week. One year has 52 full veeks with 7 days.  
The exception is the last 53rd veek that has only 1 celebration day. For leap years there are 2 celebration days.  
The new year always starts with `01v 1d` - short pronunciation one-vee one-dee. Basically every year starts with a monday.  
Veek-days are similar to week-days, but without names. They use just numbers and the `d` unit.  
`1d` is the new name for Monday and `7d` is the new name for Sunday.  
The veek_date format is global for every language and looks exactly like this:  
`2021c 52v 2d`  
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

PWA wasm converter and lengthy explanation:  
<https://github.com/LucianoBestia/new_date_time_units_and_formats/>  
PWA wasm clock:  
<https://github.com/LucianoBestia/veeks_millis_clock>  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

[comment]: # (lmake_md_to_doc_comments segment end A)
