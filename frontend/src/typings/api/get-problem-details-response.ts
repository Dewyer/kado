/* tslint:disable */
import { ProblemFullyPopulatedDto } from './problem-fully-populated-dto';
import { SubmissionDto } from './submission-dto';

export interface GetProblemDetailsResponse {
  problem: ProblemFullyPopulatedDto;
  submissions: Array<SubmissionDto>;
}
