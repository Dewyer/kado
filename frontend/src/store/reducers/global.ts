import * as actions from "../actions";
import { GlobalActionTypes, GlobalState } from "../store.types";

export const initialState: GlobalState = {
	user: undefined,
	loading: false,
};

const resultReducer = (state = initialState, action: GlobalActionTypes): GlobalState => {
	switch (action.type) {
		case actions.LOG_OUT:
			return { ...state, user: undefined, };
		default:
			return state;
	}
};

export default resultReducer;
