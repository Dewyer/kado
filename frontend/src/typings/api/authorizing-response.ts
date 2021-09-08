/* tslint:disable */
import { UserDto } from './user-dto';

export interface AuthorizingResponse {
  access_token: string;
  refresh_token: string;
  user: UserDto;
}
