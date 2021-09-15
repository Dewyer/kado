/* tslint:disable */
export interface SubmissionDto {
  correct?: null | boolean;
  finished_at?: null | number;
  id: string;
  owner_id: string;
  problem_id: string;
  sample_index?: null | number;
  started_at: number;
  test_count: number;
}
