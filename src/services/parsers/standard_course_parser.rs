use crate::{
    models::{CourseParseFormat, CourseRecord, CourseRecordType},
    services::course_manager::CourseManager,
};
use chrono::{Duration, NaiveTime, Timelike, Weekday};
use regex::Regex;
use std::{rc::Rc, usize};

const MIN_HOUR: u32 = 8;

const COURSE_RECORD_REGEX: &str = r#"(?x)
    <td>__([^_]+)__(?:[^<]*)<\/td>   # 1: Code
    <td>([^<]*)<\/td>                # 2: Name
    <td>([^<]*)<\/td>                # 3: Group
    <td>([^<]*)<\/td>                # 4: Type
    <td>([^<]*)<\/td>                # 5: Day
    <td>([^<]*)<\/td>                # 6: From
    <td>([^<]*)<\/td>                # 7: To
    <td>([^<]*)<\/td>                # 8: Class size
    <td>([^<]*)<\/td>                # 9: Enrolled
    <td>([^<]*)<\/td>                # 10: Waiting
    <td>([^<]*)<\/td>                # 11: Status
    <td>([^<]*)<\/td>                # 12: Location
"#;

pub fn parse(course_manager: &mut CourseManager, data: &str) {
    // Clear existing data
    course_manager.course_records.clear();
    course_manager.course_definitions.clear();

    // Parse new data
    let re = Regex::new(COURSE_RECORD_REGEX).unwrap();
    for c in re.captures_iter(data) {
        let mut code = get_capture_value(&c, 1);

        // Fixup name
        let name_fixup = get_capture_value(&c, 2).replace("&amp;", "&");
        let mut name = name_fixup.as_str();

        // Parse group with potential irregular format
        // Auto fixes code and name if needed
        let group_str = get_capture_value(&c, 3);
        let (group, parse_format) = parse_group(&mut code, &mut name, group_str);

        // Everything else
        let record_type: CourseRecordType = parse_direct(&c, 4);
        let day: Weekday = parse_direct(&c, 5);
        let mut from: NaiveTime = parse_direct(&c, 6);
        let mut to: NaiveTime = parse_direct(&c, 7);
        let class_size: i32 = parse_direct(&c, 8);
        let enrolled: i32 = parse_direct(&c, 9);
        let waiting: i32 = parse_direct(&c, 10);
        let status = sanitize_str(get_capture_value(&c, 11));
        let location = sanitize_str(get_capture_value(&c, 12));

        // Validate timespans
        fix_timespan(&mut from);
        fix_timespan(&mut to);

        // Get course def and register record
        let course_definition_rc = course_manager.get_or_add_course_definition(code, name);

        // Update course stats
        match record_type {
            CourseRecordType::Lecture => course_definition_rc.borrow_mut().lecture_count += 1,
            CourseRecordType::Tutorial => course_definition_rc.borrow_mut().tutorial_count += 1,
            CourseRecordType::None => {
                panic!("Invalid course type {:?}", course_definition_rc.borrow())
            }
        }

        course_manager.course_records.push(CourseRecord::new(
            Rc::clone(&course_definition_rc),
            group,
            record_type,
            day,
            from,
            to,
            class_size,
            enrolled,
            waiting,
            status,
            location,
            parse_format,
        ));
    }
}

fn fix_timespan(timespan: &mut NaiveTime) {
    if timespan.hour() < MIN_HOUR {
        *timespan = *timespan + Duration::hours(12);
    }
}

fn parse_group<'a>(
    code: &mut &'a str,
    name: &mut &'a str,
    group_str: &'a str,
) -> (i32, CourseParseFormat) {
    let group: i32;
    let mut parse_format: CourseParseFormat;

    // Irregular format detection
    let has_irregular_format =
        *code == "LECS000" || *code == "TUTS000" || group_str.contains(*code);
    if has_irregular_format {
        // I forgot how this used to work so im copying the c# impl lmao

        // Determine format
        // x-yyyy
        let sep = group_str.find('-').unwrap_or(usize::MAX);
        if group_str.len() < 9 || sep == usize::MAX {
            // Assuming group < 10
            panic!("Irregular format is invalid");
        }

        // Which format?
        //	1-MTHS002
        //  MDPS478-Vehicle System Dynamics and Control- 3

        // Okay
        // Find next
        if let Some(sep2) = group_str[sep + 1..].rfind('-') {
            parse_format = CourseParseFormat::IrregularWithName;
            *code = &group_str[..sep];
            *name = &group_str[(sep + 1)..sep2];
            group = group_str[(sep + sep2)..].parse::<i32>().unwrap();
        } else {
            //	5-MTHS003
            //	INTS203-G.1
            match group_str[..sep].parse::<i32>() {
                Ok(potential_group) => {
                    parse_format = CourseParseFormat::IrregularWithoutName;
                    group = potential_group;
                    *code = &group_str[(sep + 1)..];

                    //	5-5MTHS003
                    if code.chars().next().unwrap().is_ascii_digit() {
                        parse_format = CourseParseFormat::IrregularWithoutNameGroupPrefixed;
                        *code = &(*code)[1..];
                    }
                }
                Err(_) => {
                    //	INTS203-G.1
                    parse_format = CourseParseFormat::IrregularWithoutNameGroupPostFixed;
                    *code = &group_str[..sep];

                    // Crazy
                    let group_part = group_str[..(sep + 1)].replace("G.", "");
                    match group_part.parse::<i32>() {
                        Ok(g) => group = g,
                        Err(_) => {
                            parse_format = CourseParseFormat::IrregularWithNameNoGroup;
                            group = -1;
                            *name = &group_str[(sep + 1)..];
                        }
                    }
                }
            }
        }
    } else {
        group = group_str.parse::<i32>().unwrap();
        parse_format = CourseParseFormat::Standard;
    }

    (group, parse_format)
}

fn sanitize_str(data: &str) -> String {
    data.trim().replace("_", "")
}

fn get_capture_value<'a>(c: &'a regex::Captures<'a>, idx: usize) -> &'a str {
    c.get(idx).unwrap().as_str().trim()
}

fn parse_direct<T>(c: &regex::Captures, idx: usize) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    sanitize_str(get_capture_value(c, idx))
        .parse::<T>()
        .unwrap()
}
