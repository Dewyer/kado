/* tslint:disable */
import { ErrorResponse } from './error-response';
import { GetProblemsResponse } from './get-problems-response';

export type ProblemsResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? GetProblemsResponse | ErrorResponse
    : any
  : any;
