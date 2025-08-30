use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use chrono_tz::Arctic::Longyearbyen;
use std::ops::{Add, Sub};

fn main() {
    let anyday = NaiveDate::from_ymd_opt(1991, 4, 12);
    println!("anyday {:?}", anyday.unwrap());
    let anyday = "1991-04-12";
    let anyday = anyday.parse::<NaiveDate>().expect("Unable to parse date");
    println!("anyday from string {:?}", anyday);

    let five_seconds = TimeDelta::new(5, 0);
    println!("{:?}", five_seconds);
    let five_minutes = TimeDelta::minutes(5);
    println!("five minutes {:?}", five_minutes);
    let five_weeks = TimeDelta::weeks(5);
    println!("five weeks {}", five_weeks);
    println!("number of days in five weeks {}", five_weeks.num_days());
    println!("number of hours in five weeks {}", five_weeks.num_hours());
    println!(
        "number of minutes in five weeks {}",
        five_weeks.num_minutes()
    );

    let total_duration = five_weeks + five_minutes + five_seconds.unwrap();
    println!(
        "total duration calculated from adding above TimeDeltas {}",
        total_duration
    );
    println!(
        "number of microseconds in the total duration {:?}",
        total_duration.num_microseconds()
    );

    println!(
        "5 day after anyday variable {}",
        anyday.add(TimeDelta::days(5))
    );

    println!(
        "2 weeks and 5 days after anyday var {}",
        anyday.add(TimeDelta::weeks(2) + TimeDelta::days(5))
    );

    println!(
        "1 week 2 days prior to anyday var {}",
        anyday.sub(TimeDelta::weeks(1) + TimeDelta::days(2))
    );

    let four_thirty_am = NaiveTime::from_hms_opt(4, 30, 0);
    println!("four_thirty_am {:?}", four_thirty_am.unwrap());

    let date = NaiveDate::from_ymd_opt(1969, 7, 20).unwrap();
    let time = NaiveTime::from_hms_opt(20, 17, 0).unwrap();
    let moon_landing = NaiveDateTime::new(date, time);
    println!("moon_landing {}", moon_landing);

    let system_time = Local::now();
    // let utc_time = Utc::now();
    let system_utc_time = system_time.with_timezone(&Utc);
    let system_arctic_time = system_time.with_timezone(&Longyearbyen);
    println!(
        "local time {}, utc time {}, arctic time {}",
        system_time, system_utc_time, system_arctic_time
    );

    let someday = "31-Oct-2000 18:07:54 -0600";
    let someday_dt = DateTime::parse_from_str(someday, "%d-%b-%Y %H:%M:%S %z");
    println!("parse from string {:?}", someday_dt);
    println!(
        "format datetime to string {}",
        someday_dt.unwrap().format("%m-%b-%Y")
    );
}
