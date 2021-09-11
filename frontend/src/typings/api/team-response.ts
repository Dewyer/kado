/* tslint:disable */
import { ErrorResponse } from './error-response';
import { GetUsersTeamResponse } from './get-users-team-response';

export type TeamResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? GetUsersTeamResponse | ErrorResponse
    : any
  : any;
