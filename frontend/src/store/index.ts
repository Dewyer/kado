import { createStore, combineReducers, applyMiddleware } from "redux";
import { composeWithDevTools } from "redux-devtools-extension";
import ReduxThunk from "redux-thunk";
import globalReducer from "./reducers/global";

const rootReducer = combineReducers({
	global: globalReducer,
});

export type RootState = ReturnType<typeof rootReducer>;

export const store = createStore(rootReducer, composeWithDevTools(applyMiddleware(ReduxThunk)));
