mod workout;
mod serializer;

pub use workout::{Health, HealthShort, WorkoutResult, WorkoutType};
pub use serializer::sql_serializer;

#[cfg(test)]
mod app_tests {
    use super::*;
    use chrono::offset::Utc;

    #[test]
    fn create_workout_result() {
        WorkoutResult::new(
            Some(Utc::now().to_string()), 
            10, 
            WorkoutType::PullUp, 
            Health::new(HealthShort::Bad, Some("not really bad"))
        );
    }

    #[test]
    #[ignore = "empty"]
    fn serialize_workout_to_sql() {

    }
}
