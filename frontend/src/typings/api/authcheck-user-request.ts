/* tslint:disable */
import { CheckUserRequest } from './check-user-request';

export type AuthcheckUserRequest<
  TCode extends 'application/json' = 'application/json'
> = TCode extends 'application/json' ? CheckUserRequest : any;
