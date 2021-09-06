import { ReactElement } from "react";
import { Link } from "react-router-dom";
import { GLOBAL_ROUTES } from "../../routing/routingConstants";
import "./Logo.scss";

export function Logo(): ReactElement {
	return (
		<Link className="logo" to={GLOBAL_ROUTES.HOME}>
			<span className="logo-primary">Kado</span>
			<span className="logo-secondary">Lol</span>
		</Link>
	);
}
