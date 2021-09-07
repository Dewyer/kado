import { PropsWithChildren, ReactElement } from "react";
import { Redirect, Route, RouteProps } from "react-router-dom";

export interface ProtectedRouteProps extends RouteProps {
	restrictedPath?: string;
}

export const ProtectedRoute = (props: PropsWithChildren<ProtectedRouteProps>): ReactElement => {
	const { restrictedPath, children, ...rest } = props;

	const isLoggedIn = false;

	return isLoggedIn ? (
		<Route {...rest}>{children}</Route>
	) : (
		<Route {...rest}>
			<Redirect to={{ pathname: restrictedPath || "/" }} />
		</Route>
	);
};
