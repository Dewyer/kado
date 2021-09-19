/* tslint:disable */
import { UserLeaderboardEntryDto } from './user-leaderboard-entry-dto';

export interface GetIndividualLeaderboardResponse {
  leaderboard: Array<UserLeaderboardEntryDto>;
  page_count: number;
  user_ranking: UserLeaderboardEntryDto;
}
