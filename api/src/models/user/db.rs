use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::users;
use crate::models::user::{UserDto, UserLeaderboardEntryDto};
use crate::models::team::Team;

#[derive(Queryable, Serialize, AsChangeset)]
#[table_name = "users"]
#[changeset_options(treat_none_as_null = "true")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub authenticator: String,
    pub participate_in_leaderboards: bool,
    pub individual_points: i64,
    pub last_gained_points_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub team_id: Option<Uuid>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub authenticator: &'a str,
    pub participate_in_leaderboards: bool,
    pub individual_points: i64,
    pub last_gained_points_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub team_id: Option<Uuid>,
}

impl User {
    pub fn to_dto(&self) -> UserDto {
        UserDto {
            id: self.id.to_string(),
            username: self.username.clone(),
            email: self.email.clone(),
            participate_in_leaderboards: self.participate_in_leaderboards.clone(),
            individual_points: self.individual_points,
            last_gained_points_at: self.last_gained_points_at.map(|dt| dt.to_string()),
            is_admin: self.is_admin.clone(),
            team_id: self.team_id.map(|el| el.to_string()),
        }
    }

    pub fn to_leaderboard_dto(&self, rank: usize, team: Option<Team>) -> UserLeaderboardEntryDto {
        UserLeaderboardEntryDto {
            id: self.id.to_string(),
            rank,
            username: self.username.clone(),
            individual_points: self.individual_points,
            team_name: team.map(|tt| tt.name.clone()),
        }
    }
}
