/* tslint:disable */
import { ErrorResponse } from './error-response';
import { ExchangeGithubCodeResponse } from './exchange-github-code-response';

export type AuthexchangeGithubCodeResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? ExchangeGithubCodeResponse | ErrorResponse
    : any
  : any;
