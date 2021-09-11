/* tslint:disable */
import { LoginRequest } from './login-request';

export type AuthloginRequest<
  TCode extends 'application/json' = 'application/json'
> = TCode extends 'application/json' ? LoginRequest : any;
