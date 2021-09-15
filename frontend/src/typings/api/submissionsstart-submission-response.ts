/* tslint:disable */
import { ErrorResponse } from './error-response';
import { StartSubmissionResponse } from './start-submission-response';

export type SubmissionsstartSubmissionResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? StartSubmissionResponse | ErrorResponse
    : any
  : any;
