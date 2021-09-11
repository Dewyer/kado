/* tslint:disable */
import { ErrorResponse } from './error-response';

export type TeamleaveResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? null | ErrorResponse
    : any
  : any;
