use super::parsers::standard_course_parser;
use crate::models::{CourseDefinition, CourseRecord};
use std::{cell::RefCell, rc::Rc};

pub struct CourseManager {
    pub course_definitions: Vec<Rc<RefCell<CourseDefinition>>>,
    pub course_records: Vec<CourseRecord>,
}

impl CourseManager {
    pub fn new() -> Self {
        CourseManager {
            course_definitions: Vec::new(),
            course_records: Vec::new(),
        }
    }

    pub fn get_or_add_course_definition(
        &mut self,
        code: &str,
        name: &str,
    ) -> Rc<RefCell<CourseDefinition>> {
        let pos = self
            .course_definitions
            .iter()
            .position(|c| c.borrow().code == code);

        match pos {
            Some(idx) => Rc::clone(&self.course_definitions[idx]),

            // Create new def
            None => {
                let new_def = Rc::new(RefCell::new(CourseDefinition::new(code, name))); /* Allocation here */
                self.course_definitions.push(Rc::clone(&new_def));                      /* yes, i just got to know rc */
                new_def
            }
        }
    }

    pub fn parse_courses(&mut self, data: &str) {
        standard_course_parser::parse(self, data)
    }
}
