use postgres::{ Error as PostgressError };

mod common;

use common::{ get_client };

#[cfg(test)]
mod postgres_integration {
    use super::*;

    #[test]
    fn connect() -> Result<(), PostgressError> {
        match get_client() {
            Ok(client) => {
                if !client.is_closed() {
                    client.close()?;
                }
                Ok(())
            },
            Err(err) => Err(err)
        }
    }

    #[test]
    fn insert_row() -> Result<(), PostgressError> {
        match get_client() {
            Ok(mut cl) => {
                match cl.execute("INSERT INTO workout_result (workout_type, repeats, health, health_description) VALUES ('1', '10', '2', 'wonderful')", &[]) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err)
                }
            },
            Err(err) => Err(err)
        }
    }
}