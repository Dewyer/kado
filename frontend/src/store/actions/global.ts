import * as actions from "../actions";
import { GlobalActionTypes } from "../store.types";

export const logoutAction = (): GlobalActionTypes => ({ type: actions.LOG_OUT });
