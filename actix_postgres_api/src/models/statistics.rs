use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserRoleStatistics {
    pub role: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct RegistrationStatistics {
    pub last_24h: i64,
    pub last_7d: i64,
    pub last_30d: i64,
}

#[derive(Debug, Serialize)]
pub struct UserStatistics {
    pub roles: Vec<UserRoleStatistics>,
    pub inactive_count: i64,
    pub registration_stats: RegistrationStatistics,
}