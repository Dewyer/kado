/* tslint:disable */
import { ProblemStatementDto } from './problem-statement-dto';

export interface ProblemFullyPopulatedDto {
  available_from?: null | string;
  available_until?: null | string;
  base_point_value: number;
  code_name: string;
  difficulty: number;
  id: string;
  is_deleted: boolean;
  name: string;
  problem_statement: ProblemStatementDto;
}
