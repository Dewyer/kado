/* tslint:disable */
import { StartSubmissionRequest } from './start-submission-request';

export type SubmissionsstartSubmissionRequest<
  TCode extends 'application/json' = 'application/json'
> = TCode extends 'application/json' ? StartSubmissionRequest : any;
