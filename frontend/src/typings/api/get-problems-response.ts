/* tslint:disable */
import { ProblemDto } from './problem-dto';

export interface GetProblemsResponse {
  next_problem_available_at?: null | string;
  problems: Array<ProblemDto>;
}
