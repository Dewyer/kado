/* tslint:disable */
import { LeaveTeamRequest } from './leave-team-request';

export type TeamleaveRequest<
  TCode extends 'application/json' = 'application/json'
> = TCode extends 'application/json' ? LeaveTeamRequest : any;
