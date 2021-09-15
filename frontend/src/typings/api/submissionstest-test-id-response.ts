/* tslint:disable */
import { ErrorResponse } from './error-response';
import { SendTestOutputResponse } from './send-test-output-response';

export type SubmissionstestTestIdResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? SendTestOutputResponse | ErrorResponse
    : any
  : any;
