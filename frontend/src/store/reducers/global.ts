import * as actions from "../actions";
import { GlobalActionTypes, GlobalState } from "../store.types";
import {ApiService} from "../../api/ApiService";

export const initialState: GlobalState = {
	user: undefined,
	loadedUserInitially: false,
};

const resultReducer = (state = initialState, action: GlobalActionTypes): GlobalState => {
	switch (action.type) {
		case actions.LOG_OUT:
			ApiService.deleteStoredCredentials();
			return { ...state, user: undefined, };
		case actions.AUTHORIZED:
			ApiService.storeCredentials(action.authorizingResponse);
			return {
				...state,
				user: action.authorizingResponse.user,
			};
		case actions.USER_INITIAL_LOADED:
			console.log('Inital user loaded: ', action.user);
			return {
				...state,
				user: action.user,
				loadedUserInitially: true,
			};
		default:
			return state;
	}
};

export default resultReducer;
