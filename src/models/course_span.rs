use crate::models::CourseRecord;
use chrono::{NaiveTime, Timelike};
use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct CourseSpan {
    map: HashMap<NaiveTime, Vec<Rc<RefCell<CourseRecord>>>>,
    min_from: Option<NaiveTime>,
    max_to: Option<NaiveTime>,
}

impl CourseSpan {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            min_from: None,
            max_to: None,
        }
    }

    pub fn insert_course_record(&mut self, record: &Rc<RefCell<CourseRecord>>) {
        let CourseRecord {
            start_time,
            end_time,
            ..
        } = *record.borrow();

        self.entry(start_time)
            .or_insert(Vec::new())
            .push(Rc::clone(record));

        if self.min_from.is_none() || start_time < self.min_from.unwrap() {
            self.min_from = Some(start_time);
        }

        if self.max_to.is_none() || end_time > self.max_to.unwrap() {
            self.max_to = Some(end_time);
        }
    }

    pub fn get_period_count(&self) -> u32 {
        // 8->8:50 period
        // 9->9:50 period, etc

        if self.min_from.is_none() || self.max_to.is_none() {
            return 0;
        }

        self.max_to.unwrap().hour() - self.min_from.unwrap().hour() + 1
    }
}

impl Deref for CourseSpan {
    type Target = HashMap<NaiveTime, Vec<Rc<RefCell<CourseRecord>>>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for CourseSpan {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
