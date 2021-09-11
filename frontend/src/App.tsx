import React, {useEffect} from "react";
import Routes from "src/routing/Routes";
import "src/App.scss";
import {useDispatch} from "react-redux";
import {userInitiallyLoadedAction} from "./store/actions/global";
import {ApiService} from "./api/ApiService";

const App: React.FC = () => {
	const dispatch  = useDispatch();

	useEffect(() => {
		async function refreshUser() {
			try {
				if (!ApiService.hasStoredCredentials()) {
					dispatch(userInitiallyLoadedAction(undefined));
				} else {
					const refreshResult = await ApiService.refreshCredentials();
					dispatch(userInitiallyLoadedAction(refreshResult.user));
				}
			} catch (_err) {
				dispatch(userInitiallyLoadedAction(undefined));
			}
		}

		refreshUser().catch(() => {});
	}, []);

	return (
		<Routes />
	);
}

export default App;
