/* tslint:disable */
import { CheckUserRequest } from './check-user-request';

export type RoutesAuthRoutesCheckUserRequest<
  TCode extends 'application/json' = 'application/json'
> = TCode extends 'application/json' ? CheckUserRequest : any;
