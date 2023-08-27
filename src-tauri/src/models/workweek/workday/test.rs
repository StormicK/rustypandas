use super::*;

#[test]
fn test_set_and_get_week_number() {
    let mut workday = WorkDay::new();
    workday.set_week_number(35).unwrap();
    assert_eq!(*workday.get_week_number().unwrap(), 35);
}

#[test]
fn test_set_and_get_start_time() {
    let mut workday = WorkDay::new();
    let start_time = NaiveTime::from_hms_opt(9, 0, 0);
    workday.set_start_time(start_time.unwrap()).unwrap();
    assert_eq!(*workday.get_start_time().unwrap(), start_time.unwrap());
}

#[test]
fn test_set_and_get_end_time() {
    let mut workday = WorkDay::new();
    let end_time = NaiveTime::from_hms_opt(17, 30, 0);
    workday.set_end_time(end_time.unwrap()).unwrap();
    assert_eq!(*workday.get_end_time().unwrap(), end_time.unwrap());
}

#[test]
fn test_set_and_get_pause_time() {
    let mut workday = WorkDay::new();
    let pause_time = Duration::minutes(30);
    workday.set_pause_time(pause_time).unwrap();
    assert_eq!(*workday.get_pause_time().unwrap(), pause_time);
}

#[test]
fn test_calculate_hours_worked() {
    let mut workday = WorkDay::new();
    let start_time = NaiveTime::from_hms_opt(9, 0, 0);
    let end_time = NaiveTime::from_hms_opt(17, 30, 0);
    let pause_time = Duration::minutes(30);

    workday.set_start_time(start_time.unwrap()).unwrap();
    workday.set_end_time(end_time.unwrap()).unwrap();
    workday.set_pause_time(pause_time).unwrap();

    let hours_worked = workday.get_hours_worked().unwrap();
    println!("{:?}", hours_worked);
    assert_eq!(hours_worked, Duration::hours(8));// + Duration::minutes(30));
}

#[test]
fn test_negative_hours_worked() {
    let mut workday = WorkDay::new();
    let start_time = NaiveTime::from_hms_opt(9, 0, 0);
    let end_time = NaiveTime::from_hms_opt(8, 0, 0); // Invalid end time (before start time)
    let pause_time = Duration::minutes(30);

    workday.set_start_time(start_time.unwrap()).unwrap();
    workday.set_end_time(end_time.unwrap()).unwrap();
    workday.set_pause_time(pause_time).unwrap();

    let result = workday.get_hours_worked();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ModelError::ConfigurationFailedError());
}
