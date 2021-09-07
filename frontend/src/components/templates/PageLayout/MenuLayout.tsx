import React from "react";
import styles from "./PageLayout.module.scss";
import classNames from "classnames";
import {Logo} from "../../atoms/Logo/Logo";
import { Link } from "react-router-dom";
import {GLOBAL_ROUTES} from "../../../routing/routingConstants";

export const MenuLayout: React.FC = () => {
    return <div className={classNames(styles.MenuLayout, "ui secondary pointing menu")}>
        <Logo className={"item"} />
        <Link to={GLOBAL_ROUTES.HOME} className="item">
            Home
        </Link>
        <Link to={GLOBAL_ROUTES.HOME} className="item">
            Messages
        </Link>
        <Link to={GLOBAL_ROUTES.HOME} className="item active">
            Friends
        </Link>
        <div className="right menu">
            <a className="ui item">
                Logout
            </a>
        </div>
    </div>;
};