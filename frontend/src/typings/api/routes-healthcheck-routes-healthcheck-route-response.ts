/* tslint:disable */
export type RoutesHealthcheckRoutesHealthcheckRouteResponse<
  TCode extends 200 = 200,
  TContentType extends 'application/json' = 'application/json'
> = TCode extends 200
  ? TContentType extends 'application/json'
    ? null
    : any
  : any;
