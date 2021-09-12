/* tslint:disable */
import { ErrorResponse } from './error-response';
import { GetApiTokenResponse } from './get-api-token-response';

export type ApiTokenResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? GetApiTokenResponse | ErrorResponse
    : any
  : any;
