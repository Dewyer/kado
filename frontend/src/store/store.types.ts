import { AxiosError } from "axios";
import * as actions from "./actions";

export interface LogoutAction {
	type: typeof actions.LOG_OUT;
}

export interface GlobalState {
	loading: boolean;
}

export type GlobalActionTypes = LogoutAction;
