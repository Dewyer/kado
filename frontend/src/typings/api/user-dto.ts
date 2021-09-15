/* tslint:disable */
export interface UserDto {
  email: string;
  id: string;
  individual_points: number;
  is_admin: boolean;
  participate_in_leaderboards: boolean;
  team_id?: null | string;
  username: string;
}
