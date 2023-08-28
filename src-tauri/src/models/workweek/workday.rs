use crate::models::errors::ModelError;
use chrono::{NaiveTime, Duration};

pub trait WorkDayModelTrait {
    fn set_week_number(&mut self, week_number: u32) -> Result<(), ModelError>;
    fn get_week_number(&self) -> Result<&u32, ModelError>;

    fn set_start_time(&mut self, start_time: NaiveTime) -> Result<(), ModelError>;
    fn get_start_time(&self) -> Result<&NaiveTime, ModelError>;

    fn set_end_time(&mut self, end_time: NaiveTime) -> Result<(), ModelError>;
    fn get_end_time(&self) -> Result<&NaiveTime, ModelError>;

    fn set_pause_time(&mut self, pause_time: Duration) -> Result<(), ModelError>;
    fn get_pause_time(&self) -> Result<&Duration, ModelError>;

    fn get_hours_worked(&self) -> Result<Duration, ModelError>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkDay {
    week_number: u32,
    start_time: NaiveTime,
    end_time: NaiveTime,
    pause_time: Duration,
}

impl WorkDay {
    pub fn new() -> WorkDay {
        WorkDay {
            week_number: 0,
            start_time: NaiveTime::from_hms(0, 0, 0),
            end_time: NaiveTime::from_hms(0, 0, 0),
            pause_time: Duration::zero(),
        }
    }
}

impl WorkDayModelTrait for WorkDay {
    fn set_week_number(&mut self, week_number: u32) -> Result<(), ModelError> {
        self.week_number = week_number;
        Ok(())
    }

    fn get_week_number(&self) -> Result<&u32, ModelError> {
        Ok(&self.week_number)
    }

    fn set_start_time(&mut self, start_time: NaiveTime) -> Result<(), ModelError> {
        self.start_time = start_time;
        Ok(())
    }

    fn get_start_time(&self) -> Result<&NaiveTime, ModelError> {
        Ok(&self.start_time)
    }

    fn set_end_time(&mut self, end_time: NaiveTime) -> Result<(), ModelError> {
        self.end_time = end_time;
        Ok(())
    }

    fn get_end_time(&self) -> Result<&NaiveTime, ModelError> {
        Ok(&self.end_time)
    }

    fn set_pause_time(&mut self, pause_time: Duration) -> Result<(), ModelError> {
        self.pause_time = pause_time;
        Ok(())
    }

    fn get_pause_time(&self) -> Result<&Duration, ModelError> {
        Ok(&self.pause_time)
    }

    fn get_hours_worked(&self) -> Result<Duration, ModelError> {
        // Calculate the hours worked by subtracting the end time from the start time
        let hours_worked = self.end_time.signed_duration_since(self.start_time);
        
        // Subtract the pause time from the total hours worked
        let hours_worked_without_pause = hours_worked - self.pause_time;

        if hours_worked_without_pause < Duration::zero() {
            return Err(ModelError::ConfigurationFailedError());
        }

        Ok(hours_worked_without_pause)
    }
}

#[cfg(test)]
mod test;