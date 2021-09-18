/* tslint:disable */
import { ErrorResponse } from './error-response';
import { GetTeamLeaderboardResponse } from './get-team-leaderboard-response';

export type LeaderboardteamResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? GetTeamLeaderboardResponse | ErrorResponse
    : any
  : any;
