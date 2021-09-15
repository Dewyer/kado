/* tslint:disable */
export interface UserDto {
  email: string;
  id: string;
  individual_points: number;
  is_admin: boolean;
  last_gained_points_at?: null | string;
  participate_in_leaderboards: boolean;
  team_id?: null | string;
  username: string;
}
