/* tslint:disable */
import { AuthorizingResponse } from './authorizing-response';
import { ErrorResponse } from './error-response';

export type AuthloginResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? AuthorizingResponse | ErrorResponse
    : any
  : any;
