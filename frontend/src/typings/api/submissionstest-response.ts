/* tslint:disable */
import { ErrorResponse } from './error-response';
import { GetTestInputResponse } from './get-test-input-response';

export type SubmissionstestResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? GetTestInputResponse | ErrorResponse
    : any
  : any;
