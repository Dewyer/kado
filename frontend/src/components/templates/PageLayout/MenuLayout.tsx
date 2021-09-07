import React from "react";
import styles from "./PageLayout.module.scss";
import classNames from "classnames";
import {Logo} from "src/components/atoms/Logo/Logo";
import { Link } from "react-router-dom";
import {GLOBAL_ROUTES} from "src/routing/routingConstants";
import {useDispatch, useSelector} from "react-redux";
import {makeSelectLoggedIn} from "src/store/selectors/global.selectors";
import {logoutAction} from "src/store/actions/global";

export const MenuLayout: React.FC = () => {
    const dispatch = useDispatch();
    const loggedIn = useSelector(makeSelectLoggedIn());

    const onLogout = () => {
        dispatch(logoutAction());
    }

    return <div className={classNames(styles.MenuLayout, "ui secondary pointing menu")}>
        <Logo className={"item"} />
        <Link to={GLOBAL_ROUTES.HOME} className="item">
            Home
        </Link>
        {loggedIn && (
            <>
                <Link to={GLOBAL_ROUTES.HOME} className="item">
                    Messages
                </Link>
                <Link to={GLOBAL_ROUTES.HOME} className="item active">
                Friends
                </Link>
            </>
        )}
        <div className="right menu">
            {loggedIn ?
                <a onClick={onLogout} className="ui item">
                    Logout
                </a>
                :
                <Link to={GLOBAL_ROUTES.LOGIN} className="ui item">
                    Login
                </Link>
            }
        </div>
    </div>;
};