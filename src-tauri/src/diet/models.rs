use serde::{Deserialize, Serialize};

/// Meal type enum for categorizing diet entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
    Other,
}

impl std::fmt::Display for MealType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MealType::Breakfast => write!(f, "Breakfast"),
            MealType::Lunch => write!(f, "Lunch"),
            MealType::Dinner => write!(f, "Dinner"),
            MealType::Snack => write!(f, "Snack"),
            MealType::Other => write!(f, "Other"),
        }
    }
}

impl std::str::FromStr for MealType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Breakfast" => Ok(MealType::Breakfast),
            "Lunch" => Ok(MealType::Lunch),
            "Dinner" => Ok(MealType::Dinner),
            "Snack" => Ok(MealType::Snack),
            "Other" => Ok(MealType::Other),
            _ => Err(format!("Unknown meal type: {}", s)),
        }
    }
}

/// Diet entry as stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DietEntry {
    pub id: i64,
    pub member_id: i64,
    pub timestamp: String,
    pub meal_type: MealType,
    pub description: String,
    pub calories: Option<i64>,
    pub notes: Option<String>,
}

/// Request payload for creating a new diet entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDietEntryRequest {
    pub member_id: i64,
    pub timestamp: String,
    pub meal_type: MealType,
    pub description: String,
    pub calories: Option<i64>,
    pub notes: Option<String>,
}

/// Request payload for updating an existing diet entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDietEntryRequest {
    pub id: i64,
    pub member_id: Option<i64>,
    pub timestamp: Option<String>,
    pub meal_type: Option<MealType>,
    pub description: Option<String>,
    pub calories: Option<i64>,
    pub notes: Option<String>,
}

/// Filter criteria for querying diet entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DietEntryFilter {
    pub member_id: Option<i64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
