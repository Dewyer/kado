import { PropsWithChildren, ReactElement } from "react";
import { Redirect, Route, RouteProps } from "react-router-dom";
import {useSelector} from "react-redux";
import {makeSelectUser} from "../../store/selectors/global.selectors";

export interface ProtectedRouteProps extends RouteProps {
	restrictedPath?: string;
}

export const ProtectedRoute = (props: PropsWithChildren<ProtectedRouteProps>): ReactElement => {
	const { restrictedPath, children, ...rest } = props;

	const user = useSelector(makeSelectUser());

	return !!user ? (
		<Route {...rest}>{children}</Route>
	) : (
		<Route {...rest}>
			<Redirect to={{ pathname: restrictedPath || "/" }} />
		</Route>
	);
};
