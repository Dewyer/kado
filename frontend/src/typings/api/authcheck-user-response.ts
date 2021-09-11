/* tslint:disable */
import { CheckUserResponse } from './check-user-response';
import { ErrorResponse } from './error-response';

export type AuthcheckUserResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? CheckUserResponse | ErrorResponse
    : any
  : any;
