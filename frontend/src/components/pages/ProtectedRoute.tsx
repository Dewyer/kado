import { PropsWithChildren, ReactElement } from "react";
import { useSelector } from "react-redux";
import { Redirect, Route, RouteProps } from "react-router-dom";
import { makeSelectLoggedIn } from "../../store/selectors/global.selectors";

export interface ProtectedRouteProps extends RouteProps {
	restrictedPath?: string;
}

export const ProtectedRoute = (props: PropsWithChildren<ProtectedRouteProps>): ReactElement => {
	const { restrictedPath, children, ...rest } = props;

	const isLoggedIn = useSelector(makeSelectLoggedIn());

	return isLoggedIn ? (
		<Route {...rest}>{children}</Route>
	) : (
		<Route {...rest}>
			<Redirect to={{ pathname: restrictedPath || "/" }} />
		</Route>
	);
};
