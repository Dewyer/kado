import { createSelector } from "reselect";
import { RootState } from "src/store";
import { initialState } from "src/store/reducers/global";


const selectGlobal = (state: RootState) => state.global;

export const makeSelectLoggedIn = () => createSelector(selectGlobal, (substate) => !!substate.user);

