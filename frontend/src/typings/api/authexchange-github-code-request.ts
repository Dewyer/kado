/* tslint:disable */
import { ExchangeGithubCodeRequest } from './exchange-github-code-request';

export type AuthexchangeGithubCodeRequest<
  TCode extends 'application/json' = 'application/json'
> = TCode extends 'application/json' ? ExchangeGithubCodeRequest : any;
