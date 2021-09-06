import * as actions from "../actions";
import { GlobalActionTypes, GlobalState } from "../store.types";

export const initialState: GlobalState = {
	loading: false,
};

const resultReducer = (state = initialState, action: GlobalActionTypes): GlobalState => {
	switch (action.type) {
		case actions.LOG_OUT:
			return { ...state };
		default:
			return state;
	}
};

export default resultReducer;
