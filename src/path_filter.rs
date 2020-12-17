use std::path::{PathBuf, Path};
use chrono::{NaiveDate, Utc};
use chrono::format::ParseError;

fn extract_date(file_prefix: &String, p: &Path) -> NaiveDate {
    let fname = p.file_name().expect("could not get filename").to_str().expect("could not get str");
    NaiveDate::parse_from_str(fname, &format!("{}%Y-%m-%d.zip",file_prefix)).expect("could not parse date")
}

#[test]
fn extract_time_works() {
    let p = Path::new("/home/k/backup/dbdumps/zip/psqldump_2020-12-17.zip");
    let extracted_date = extract_date(&"psqldump_".to_string(), &p);
    assert_eq!(extracted_date,NaiveDate::from_ymd(2020, 12, 17));
}

fn is_start_of_month(d: &NaiveDate) -> bool {
    use chrono::Datelike;
    d.day() == 1
}

#[test]
fn is_start_of_month_works() {
    let notstart = NaiveDate::from_ymd(2020, 12, 17);
    let isstart = NaiveDate::from_ymd(2020, 12, 01);

    assert_eq!(is_start_of_month(&notstart), false);
    assert_eq!(is_start_of_month(&isstart), true);
}

fn is_in_current_month(d: &NaiveDate) -> bool {
    use chrono::Datelike;
    let now = Utc::now();
    now.month() == d.month()
}

// Would be pretty silly to test this next month 
// and to much work to test correctly
// 
// #[test]
// fn is_in_current_month_works() {
//     let notthismonth = NaiveDate::from_ymd(2020, 11, 17);
//     let thismonth = NaiveDate::from_ymd(2020, 12, 09);

//     assert_eq!(is_in_current_month(&notthismonth), false);
//     assert_eq!(is_in_current_month(&thismonth), true);
// }

fn is_monday(d: &NaiveDate) -> bool {
    use chrono::{Datelike, Weekday};
    d.weekday() == Weekday::Mon
}

#[test]
fn is_monday_works() {
    let notmonday = NaiveDate::from_ymd(2020, 12, 17);
    let ismonday = NaiveDate::from_ymd(2020, 12, 7);

    assert_eq!(is_monday(&notmonday), false);
    assert_eq!(is_monday(&ismonday), true);
}

fn is_last_week(d: &NaiveDate) -> bool {
    use chrono::{Datelike, Utc, Duration};
    let last_week = Utc::now().naive_utc().date() - Duration::days(7);
    last_week.lt(d)
}

// Would be pretty silly to test this next month 
// and to much work to test correctly
// 
// #[test]
// fn is_last_week_works() {
//     let thisweek = NaiveDate::from_ymd(2020, 12, 11);
//     let lastmonth = NaiveDate::from_ymd(2020, 12, 1);

//     assert_eq!(is_last_week(&lastmonth), false);
//     assert_eq!(is_last_week(&thisweek), true);
// }

pub fn filter<'a>(file_prefix: &String, pathes: &'a Vec<PathBuf>) -> Vec<&'a Path> {
//pub fn filter(pathes: &Vec<&PathBuf>) -> Vec<&Path> {
    let mut ret: Vec<&Path> = Vec::with_capacity(pathes.len());
    
    for path in pathes {
        let date = extract_date(file_prefix, &path);

        // we take all start of month files
        if is_start_of_month(&date) {
            ret.push(&path);
        }

        // if it happened this month we only take every monday
        else if is_in_current_month(&date) && is_monday(&date) {
            ret.push(&path);
        }
        
        // if it is not older than 7 days
        else if is_last_week(&date) {
            ret.push(&path);
        }
    }

    ret
}

#[test] 
fn filter_works() {
    let pathes: Vec<PathBuf> = vec![
        PathBuf::from("/psqldump_2020-08-01.zip".to_string()),
        PathBuf::from("/psqldump_2020-09-01.zip".to_string()),
        PathBuf::from("/psqldump_2020-09-15.zip".to_string()),
        PathBuf::from("/psqldump_2020-10-02.zip".to_string()),
        PathBuf::from("/psqldump_2020-12-01.zip".to_string()),
        PathBuf::from("/psqldump_2020-12-02.zip".to_string()),
        PathBuf::from("/psqldump_2020-12-03.zip".to_string()),
        PathBuf::from("/psqldump_2020-12-07.zip".to_string()),
        PathBuf::from("/psqldump_2020-12-08.zip".to_string()),
        PathBuf::from("/psqldump_2020-12-11.zip".to_string()),
        PathBuf::from("/psqldump_2020-12-17.zip".to_string())
    ];

    let correct_pathes: Vec<PathBuf> = vec![
        PathBuf::from("/psqldump_2020-08-01.zip".to_string()),
        PathBuf::from("/psqldump_2020-09-01.zip".to_string()),
        PathBuf::from("/psqldump_2020-12-01.zip".to_string()),
        PathBuf::from("/psqldump_2020-12-07.zip".to_string()),
        PathBuf::from("/psqldump_2020-12-11.zip".to_string()),
        PathBuf::from("/psqldump_2020-12-17.zip".to_string())
    ];

    assert_eq!(filter(&"psqldump_".to_string(), &pathes), correct_pathes);
}
