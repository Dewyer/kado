import { createSelector } from "reselect";
import { RootState } from "src/store";
import { initialState } from "src/store/reducers/global";


const selectGlobal = (state: RootState) => state.global;

export const makeSelectUser = () => createSelector(selectGlobal, (substate) => substate.user);

export const makeSelectUserInitiallyLoaded = () => createSelector(selectGlobal, (substate) => substate.loadedUserInitially);
