import * as actions from "../actions";
import { GlobalActionTypes, GlobalState } from "../store.types";
import {ApiService} from "../../api/ApiService";

export const initialState: GlobalState = {
	user: undefined,
	loading: false,
};

const resultReducer = (state = initialState, action: GlobalActionTypes): GlobalState => {
	switch (action.type) {
		case actions.LOG_OUT:
			return { ...state, user: undefined, };
		case actions.AUTHORIZED:
			ApiService.storeCredentials(action.authorizingResponse);

			return {
				...state,
				user: action.authorizingResponse.user,
			};
		default:
			return state;
	}
};

export default resultReducer;
