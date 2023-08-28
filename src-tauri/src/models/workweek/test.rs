use chrono::{NaiveTime, Duration};

use crate::models::workweek::{
    workday::{WorkDay, WorkDayModelTrait},
    WorkWeek, WorkWeekModelTrait,
};

#[test]
fn test_set_and_get_work_week() {
    let mut work_week = WorkWeek::new(42);
    assert_eq!(*work_week.get_work_week().unwrap(), 42);

    work_week.set_work_week(43).unwrap();
    assert_eq!(*work_week.get_work_week().unwrap(), 43);
}

#[test]
fn test_set_and_get_work_days() {
    let mut work_week = WorkWeek::new(42);
    let work_days = vec![WorkDay::new(), WorkDay::new(), WorkDay::new(), WorkDay::new(), WorkDay::new()];

    work_week.set_work_days(work_days.clone()).unwrap();
    assert_eq!(*work_week.get_work_days().unwrap(), work_days);
}

#[test]
fn test_get_hours_worked() {
    let mut work_week = WorkWeek::new(42);
    let mut work_days = vec![WorkDay::new(), WorkDay::new(), WorkDay::new(), WorkDay::new(), WorkDay::new()];

    work_days[0]
        .set_start_time(NaiveTime::from_hms_opt(9, 0, 0).unwrap())
        .unwrap();
    work_days[0]
        .set_end_time(NaiveTime::from_hms_opt(17, 30, 0).unwrap())
        .unwrap();
    work_days[0].set_pause_time(Duration::minutes(30)).unwrap();

    work_days[1]
        .set_start_time(NaiveTime::from_hms_opt(9, 0, 0).unwrap())
        .unwrap();
    work_days[1]
        .set_end_time(NaiveTime::from_hms_opt(17, 30, 0).unwrap())
        .unwrap();
    work_days[1].set_pause_time(Duration::minutes(30)).unwrap();

    work_days[2]
        .set_start_time(NaiveTime::from_hms_opt(9, 0, 0).unwrap())
        .unwrap();
    work_days[2]
        .set_end_time(NaiveTime::from_hms_opt(17, 30, 0).unwrap())
        .unwrap();
    work_days[2].set_pause_time(Duration::minutes(30)).unwrap();

    work_days[3]
        .set_start_time(NaiveTime::from_hms_opt(9, 0, 0).unwrap())
        .unwrap();
    work_days[3]
        .set_end_time(NaiveTime::from_hms_opt(17, 30, 0).unwrap())
        .unwrap();
    work_days[3].set_pause_time(Duration::minutes(30)).unwrap();

    work_days[4]
        .set_start_time(NaiveTime::from_hms_opt(9, 0, 0).unwrap())
        .unwrap();
    work_days[4]
        .set_end_time(NaiveTime::from_hms_opt(18, 30, 0).unwrap())
        .unwrap();
    work_days[4].set_pause_time(Duration::minutes(30)).unwrap();

    work_week.set_work_days(work_days).unwrap();
    let total_hours = work_week.get_hours_worked().unwrap();
    assert_eq!(total_hours, Duration::hours(41));
    let overtime = work_week.get_hours_worked_overtime().unwrap();
    assert_eq!(overtime, Duration::hours(1));
}
