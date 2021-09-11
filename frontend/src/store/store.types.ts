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

export interface UserInitiallyLoaded {
	type: typeof actions.USER_INITIAL_LOADED;
	user?: UserDto;
}

export interface GlobalState {
	user?: UserDto;
	loadedUserInitially: boolean;
}

export type GlobalActionTypes = LogoutAction | AuthorizedAction | UserInitiallyLoaded;
