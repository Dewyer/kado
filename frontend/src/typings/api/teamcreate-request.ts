/* tslint:disable */
import { CreateTeamRequest } from './create-team-request';

export type TeamcreateRequest<
  TCode extends 'application/json' = 'application/json'
> = TCode extends 'application/json' ? CreateTeamRequest : any;
