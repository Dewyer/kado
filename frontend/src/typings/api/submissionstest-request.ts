/* tslint:disable */
import { GetTestInputRequest } from './get-test-input-request';

export type SubmissionstestRequest<
  TCode extends 'application/json' = 'application/json'
> = TCode extends 'application/json' ? GetTestInputRequest : any;
