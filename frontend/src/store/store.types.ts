import { AxiosError } from "axios";
import * as actions from "./actions";
import {AuthorizingResponse, UserDto} from "../typings/api";

export interface LogoutAction {
	type: typeof actions.LOG_OUT;
}

export interface AuthorizedAction {
	type: typeof actions.AUTHORIZED;
	authorizingResponse: AuthorizingResponse,
}

export interface GlobalState {
	user?: UserDto;
	loading: boolean;
}

export type GlobalActionTypes = LogoutAction | AuthorizedAction;
