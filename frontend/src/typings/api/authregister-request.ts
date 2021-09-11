/* tslint:disable */
import { RegisterRequest } from './register-request';

export type AuthregisterRequest<
  TCode extends 'application/json' = 'application/json'
> = TCode extends 'application/json' ? RegisterRequest : any;
