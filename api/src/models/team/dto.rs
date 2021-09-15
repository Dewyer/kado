use crate::models::user::{UserDto, User};
use crate::models::team::Team;

#[derive(serde::Serialize, serde::Deserialize, rocket_okapi::JsonSchema)]
pub struct TeamFullyPopulatedDto {
    pub id: String,
    pub name: String,
    pub join_code: Option<String>,
    pub points: i64,
    pub owner_user: Option<UserDto>,
    pub members: Vec<UserDto>,
}

impl TeamFullyPopulatedDto {
    pub fn from_team_and_users(team: &Team, users: &Vec<User>, can_see_join_code: bool) -> TeamFullyPopulatedDto {
        let members = users.into_iter().map(|usr| usr.to_dto()).collect::<Vec<UserDto>>();
        let team_id_str = team.owner_user.map(|ow| ow.to_string());
        let owner_user = members.iter().find(|user| team_id_str.contains(&user.id)).map(|usr| usr.clone());

        TeamFullyPopulatedDto {
            id: team.id.to_string(),
            name: team.name.clone(),
            join_code: if can_see_join_code { Some(team.join_code.clone()) } else { None },
            points: team.points,
            owner_user,
            members,
        }
    }
}