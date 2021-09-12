/* tslint:disable */
import { JoinTeamRequest } from './join-team-request';

export type TeamjoinRequest<
  TCode extends 'application/json' = 'application/json'
> = TCode extends 'application/json' ? JoinTeamRequest : any;
