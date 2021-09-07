import { AxiosError } from "axios";
import * as actions from "./actions";
import {User} from "../api";

export interface LogoutAction {
	type: typeof actions.LOG_OUT;
}

export interface GlobalState {
	user?: User;
	loading: boolean;
}

export type GlobalActionTypes = LogoutAction;
