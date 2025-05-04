// models.rs
// Defines the Team struct, which holds a team's name, season, and performance stats.
/// Represents a single team’s data from the dataset, including identifying info and a vector of statistics.
/// Used as the primary unit in graph construction and similarity calculations.
#[derive(Debug, Clone)]
pub struct Team {
    pub name: String,   // Full team name plus season 
    pub season: String, // The season/year this team data is from
    pub stats: Vec<f64> // A vector of all numerical statistics associated with the team
}