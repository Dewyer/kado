use serde::Serialize;
use uuid::Uuid;
use crate::schema::teams;
use chrono::NaiveDateTime;
use crate::models::team::TeamLeaderboardEntryDto;

#[derive(Queryable, Serialize, AsChangeset)]
#[table_name = "teams"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub join_code: String,
    pub points: i64,
    pub participate_in_leaderboards: bool,
    pub last_gained_points_at: Option<NaiveDateTime>,
    pub owner_user: Option<Uuid>,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "teams"]
pub struct NewTeam<'a> {
    pub name: &'a str,
    pub join_code: &'a str,
    pub points: i64,
    pub participate_in_leaderboards: bool,
    pub last_gained_points_at: Option<NaiveDateTime>,
    pub owner_user: Option<Uuid>,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
}

impl Team {
    pub fn to_leaderboard_dto(&self, rank: usize, anonymous: bool) -> TeamLeaderboardEntryDto {
        TeamLeaderboardEntryDto {
            id: self.id.to_string(),
            rank,
            name: if anonymous { "anonymous".to_string() } else { self.name.clone() },
            points: self.points,
        }
    }
}