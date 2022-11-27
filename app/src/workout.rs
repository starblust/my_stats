
pub enum WorkoutType {
    PushUp,
    PullUp,
    Squatting,
}

impl WorkoutType {

    fn to_var(val: u8) -> Option<WorkoutType> {
        match val {
            0 => Some(WorkoutType::PushUp),
            1 => Some(WorkoutType::PullUp),
            2 => Some(WorkoutType::Squatting),
            _ => None
        }
    }

    pub fn val(&self) -> u8 {
        match &self {
            WorkoutType::PushUp => 0,
            WorkoutType::PullUp => 1,
            WorkoutType::Squatting => 2,
        }
    }
}

pub enum HealthShort {
    VeryBad,
    Bad,
    Well,
    Insane,
}

impl HealthShort {

    pub fn to_var(val: u8) -> Option<HealthShort> {
        match val {
            1 => Some(HealthShort::VeryBad),
            2 => Some(HealthShort::Bad),
            3 => Some(HealthShort::Well),
            4 => Some(HealthShort::Insane),
            _ => None,
        }
    }

    pub fn val(&self) -> u8 {
        match &self {
            HealthShort::VeryBad => 1,
            HealthShort::Bad => 2,
            HealthShort::Well => 3,
            HealthShort::Insane => 4,
        }
    }
}

pub struct Health {
    pub short: HealthShort,
    pub description: Option<&'static str>,
}

impl Health {
    pub fn new(health_short: HealthShort, description: Option<&'static str>) -> Health {
        Health {
            short: health_short,
            description
        }
    }
}

pub struct WorkoutResult {
    pub datetime: Option<String>,
    pub repeats: u8,
    pub workout_type: WorkoutType,
    pub health: Health, 
}

impl WorkoutResult {
    pub fn new(datetime: Option<String>, repeats: u8, workout_type: WorkoutType, health: Health) -> WorkoutResult {
        WorkoutResult { 
            datetime, 
            repeats,
            workout_type, 
            health, 
        }
    }
}