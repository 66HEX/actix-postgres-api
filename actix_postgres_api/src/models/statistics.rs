use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserRoleStatistics {
    pub role: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct UserStatistics {
    pub roles: Vec<UserRoleStatistics>,
    pub inactive_count: i64,
}