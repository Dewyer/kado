/* tslint:disable */
import { ErrorResponse } from './error-response';
import { GetIndividualLeaderboardResponse } from './get-individual-leaderboard-response';

export type LeaderboardindividualResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? GetIndividualLeaderboardResponse | ErrorResponse
    : any
  : any;
