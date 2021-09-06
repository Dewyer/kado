import { Route, Switch, Redirect } from "react-router-dom";
import { GLOBAL_ROUTES } from "./routingConstants";
import { HomeScreen } from "../components/pages/HomeScreen/HomeScreen";

const Routes = () => {
	return (
		<Switch>
			<Route exact path={GLOBAL_ROUTES.HOME} component={HomeScreen} />
			<Redirect to="/" />
		</Switch>
	);
};

export default Routes;
