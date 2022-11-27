use crate::workout::{ WorkoutResult };

pub mod sql_serializer {
    use super::WorkoutResult;

    pub fn to_string(workout_result: WorkoutResult) -> String {
        let mut cols: Vec<String> = vec![];
        let mut vals: Vec<String> = vec![];
        if let Some(timestamp) = workout_result.datetime {
            cols.push("datetime".into());
            vals.push(timestamp);
        }
        cols.push("repeats".into());
        vals.push(workout_result.repeats.to_string());
        cols.push("workout_type".into());
        vals.push(workout_result.workout_type.val().to_string());
        cols.push("health".into());
        vals.push(workout_result.health.short.val().to_string());
        if let Some(health_description) = workout_result.health.description {
            cols.push("health_description".into());
            vals.push(health_description.into());
        }
        format!(
            "INSERT INTO workout_result ({}) VALUES ({})", 
            cols.join(","), 
            vals.clone().into_iter().map(|s| format!("'{}'", s)).collect::<Vec<_>>().join(",") 
        )
    }
}


