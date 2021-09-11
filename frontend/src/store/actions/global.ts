import * as actions from "../actions";
import { GlobalActionTypes } from "../store.types";
import {AuthorizingResponse, UserDto} from "../../typings/api";

export const logoutAction = (): GlobalActionTypes => ({ type: actions.LOG_OUT });

export const authorizedAction = (resp: AuthorizingResponse): GlobalActionTypes => ({ type: actions.AUTHORIZED, authorizingResponse: resp, });

export const userInitiallyLoadedAction = (user?: UserDto): GlobalActionTypes =>
    ({ type: actions.USER_INITIAL_LOADED, user, });