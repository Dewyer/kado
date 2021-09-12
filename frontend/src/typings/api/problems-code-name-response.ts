/* tslint:disable */
import { ErrorResponse } from './error-response';
import { GetProblemDetailsResponse } from './get-problem-details-response';

export type ProblemsCodeNameResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? GetProblemDetailsResponse | ErrorResponse
    : any
  : any;
