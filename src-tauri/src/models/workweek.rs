mod workday;

use chrono::Duration;
use crate::models::errors::ModelError;
use workday::{WorkDay, WorkDayModelTrait};

pub trait WorkWeekModelTrait {
    fn set_work_days(&mut self, work_days: Vec<WorkDay>) -> Result<(), ModelError>;
    fn get_work_days(&self) -> Result<&Vec<WorkDay>, ModelError>;

    fn set_work_week(&mut self, work_week: u32) -> Result<(), ModelError>;
    fn get_work_week(&self) -> Result<&u32, ModelError>;

    fn get_hours_worked(&self) -> Result<Duration, ModelError>;
    fn get_hours_worked_per_week(&self) -> Result<Duration, ModelError>;
    fn get_hours_worked_overtime(&self) -> Result<Duration, ModelError>;
}

pub struct WorkWeek {
    work_days: Vec<WorkDay>,
    work_week: u32,
}

impl WorkWeek {
    pub fn new(work_week: u32) -> WorkWeek {
        WorkWeek {
            work_days: Vec::new(),
            work_week,
        }
    }
}

impl WorkWeekModelTrait for WorkWeek {
    fn set_work_days(&mut self, work_days: Vec<WorkDay>) -> Result<(), ModelError> {
        self.work_days = work_days;
        Ok(())
    }

    fn get_work_days(&self) -> Result<&Vec<WorkDay>, ModelError> {
        Ok(&self.work_days)
    }

    fn set_work_week(&mut self, work_week: u32) -> Result<(), ModelError> {
        self.work_week = work_week;
        Ok(())
    }

    fn get_work_week(&self) -> Result<&u32, ModelError> {
        Ok(&self.work_week)
    }

    fn get_hours_worked(&self) -> Result<Duration, ModelError> {
        let mut total_hours: Duration = Duration::zero();
        for workday in self.work_days.iter() {
            match workday.get_hours_worked() {
                Ok(duration) => total_hours = total_hours + duration,
                Err(_) => total_hours = total_hours + Duration::zero(),
            }
        }

        Ok(total_hours)
    }

    fn get_hours_worked_per_week(&self) -> Result<Duration, ModelError> {
        Ok(Duration::hours(40))
    }

    fn get_hours_worked_overtime(&self) -> Result<Duration, ModelError> {
        let normal_work_hours = self.get_hours_worked_per_week()?;
        let total_work_hours = self.get_hours_worked()?;
        
        if total_work_hours > normal_work_hours {
            Ok(total_work_hours - normal_work_hours)
        } else {
            Ok(Duration::zero())
        }
    }
}

#[cfg(test)]
mod test;