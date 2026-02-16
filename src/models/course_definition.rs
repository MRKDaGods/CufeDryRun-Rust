#[derive(Debug)]
pub enum CourseFlags {
    None = 0,
    MultipleLectures = 1 << 0,
    MultipleTutorials = 1 << 1,
}

#[derive(Debug)]
pub struct CourseDefinition {
    pub code: String,
    pub name: String,
    pub flags: CourseFlags, // For ykyk ;) bas we're graduating 5alas :(

    // To remove later?
    pub lecture_count: u32,
    pub tutorial_count: u32,
}

impl CourseDefinition {
    pub fn new(code: &str, name: &str) -> Self {
        Self {
            code: code.to_owned(),
            name: name.to_owned(),
            ..Default::default()
        }
    }
}

impl Default for CourseDefinition {
    fn default() -> Self {
        Self {
            code: String::from("AMMR123"),
            name: String::from("ammar wkda"),
            flags: CourseFlags::None,
            lecture_count: 0,
            tutorial_count: 0,
        }
    }
}
