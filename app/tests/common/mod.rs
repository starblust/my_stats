use postgres::{ Client, NoTls, Error as PostgressError };
use chrono::{ offset::Utc };
use app::{ WorkoutResult, Health, WorkoutType, HealthShort };

pub fn get_client() -> Result<Client, PostgressError> {
    Client::connect(
        "host=localhost user=dev password=dev dbname=my_stats",
        NoTls,
    )
}

pub fn create_workout_result() -> WorkoutResult {
    WorkoutResult::new(
        Some(Utc::now().to_string()), 
        10, 
        WorkoutType::PullUp, 
        Health::new(HealthShort::Bad, Some("not really bad"))
    )
}
