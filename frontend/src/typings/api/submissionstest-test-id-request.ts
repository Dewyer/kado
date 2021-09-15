/* tslint:disable */
import { SendTestOutputRequest } from './send-test-output-request';

export type SubmissionstestTestIdRequest<
  TCode extends 'application/json' = 'application/json'
> = TCode extends 'application/json' ? SendTestOutputRequest : any;
