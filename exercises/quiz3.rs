// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub trait ConvertGrade {
    fn convert_grade(&self) -> String;
}
pub struct ReportCard {
    pub grade: Grade,
    pub student_name: String,
    pub student_age: u8,
}

pub enum Grade {
    Numeric(f32),
    Alphabetic(String),
}

impl ConvertGrade for ReportCard {
    fn convert_grade(&self) -> String {
        match &self.grade {
            Grade::Numeric(grade) => match grade {
                g if *g >= 5.1 && *g <= 5.5 => "A+".to_string(),
                g if *g >= 4.6 && *g <= 5.0 => "A".to_string(),
                g if *g >= 4.1 && *g <= 4.5 => "A-".to_string(),
                g if *g >= 3.6 && *g <= 4.0 => "B+".to_string(),
                g if *g >= 3.1 && *g <= 3.5 => "B".to_string(),
                g if *g >= 2.6 && *g <= 3.0 => "B-".to_string(),
                g if *g >= 2.1 && *g <= 2.5 => "C+".to_string(),
                g if *g >= 1.6 && *g <= 2.0 => "C".to_string(),
                g if *g >= 1.1 && *g <= 1.5 => "C-".to_string(),
                g if *g >= 0.6 && *g <= 1.0 => "D+".to_string(),
                g if *g >= 0.1 && *g <= 0.5 => "D".to_string(),
                _ => "F".to_string(),
            },
            Grade::Alphabetic(grade) => grade.clone(),
        }
    }
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name,
            &self.student_age,
            self.convert_grade() // Calling convert_grade method to get the grade string
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::Numeric(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of C+"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: Grade::Alphabetic("A+".to_string()),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
