import { Route, Switch, Redirect } from "react-router-dom";
import { GLOBAL_ROUTES } from "./routingConstants";
import { HomePage } from "../components/pages/HomePage/HomePage";
import { LoginPage } from "../components/pages/LoginPage/LoginPage";
import {useSelector} from "react-redux";
import {makeSelectUserInitiallyLoaded} from "../store/selectors/global.selectors";

const Routes = () => {
	const userLoaded = useSelector(makeSelectUserInitiallyLoaded());

	return !userLoaded ? <p>Loading ...</p> : (
		<Switch>
			<Route exact path={GLOBAL_ROUTES.HOME} component={HomePage} />
			<Route exact path={GLOBAL_ROUTES.LOGIN} component={LoginPage} />

			<Redirect to="/" />
		</Switch>
	);
};

export default Routes;
