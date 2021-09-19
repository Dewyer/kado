/* tslint:disable */
import { UpdateProblemStatementRequest } from './update-problem-statement-request';

export type AdminupdateProblemStatementRequest<
  TCode extends 'application/json' = 'application/json'
> = TCode extends 'application/json' ? UpdateProblemStatementRequest : any;
