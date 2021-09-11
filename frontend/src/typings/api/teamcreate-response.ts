/* tslint:disable */
import { CreateTeamResponse } from './create-team-response';
import { ErrorResponse } from './error-response';

export type TeamcreateResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? CreateTeamResponse | ErrorResponse
    : any
  : any;
