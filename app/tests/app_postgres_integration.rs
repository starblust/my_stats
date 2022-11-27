// use app::{ WorkoutResult, WorkoutType, Health, HealthShort };

mod common;

use postgres::{ Error as PostgressError };
use common:: { create_workout_result, get_client };
use app::sql_serializer;

#[cfg(test)]
mod app_postgres_integration {
    use super::*;

    #[test]
    // #[ignore = "dependency is failing"]
    fn add_workout_to_db() -> Result<(), PostgressError> {
        let r = sql_serializer::to_string(create_workout_result());
        println!("sql is: {}", r);
        match get_client() {
            Ok(mut cl) => {
                match cl.execute(r.as_str(), &[]) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err)
                }
            },
            Err(err) => Err(err)
        }
    }

}