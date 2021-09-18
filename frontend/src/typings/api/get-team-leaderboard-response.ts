/* tslint:disable */
import { TeamLeaderboardEntryDto } from './team-leaderboard-entry-dto';

export interface GetTeamLeaderboardResponse {
  leaderboard: Array<TeamLeaderboardEntryDto>;
  page_count: number;
  user_team_ranking?: TeamLeaderboardEntryDto;
}
