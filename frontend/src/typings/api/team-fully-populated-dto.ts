/* tslint:disable */
import { UserDto } from './user-dto';

export interface TeamFullyPopulatedDto {
  id: string;
  join_code?: null | string;
  members: Array<UserDto>;
  name: string;
  owner_user?: UserDto;
  points: number;
}
