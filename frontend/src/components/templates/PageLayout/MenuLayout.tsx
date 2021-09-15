import React from "react";
import styles from "./PageLayout.module.scss";
import classNames from "classnames";
import {Logo} from "src/components/atoms/Logo/Logo";
import {Link, useHistory} from "react-router-dom";
import {GLOBAL_ROUTES} from "src/routing/routingConstants";
import {useDispatch, useSelector} from "react-redux";
import {makeSelectUser} from "src/store/selectors/global.selectors";
import {logoutAction} from "src/store/actions/global";
import {Dropdown} from "semantic-ui-react";

export const MenuLayout: React.FC = () => {
    const dispatch = useDispatch();
    const history = useHistory();
    const user = useSelector(makeSelectUser());
    const loggedIn = !!user;

    const onLogout = () => {
        dispatch(logoutAction());
    };

    const isTabActive = (route: string) => {
        return history.location.pathname === route;
    };

    const getTabClasses = (route: string, routeTwo?: string) => {
        return classNames("item", { active: isTabActive(route) || (routeTwo ? isTabActive(routeTwo) : false) });
    };

    return <div className={classNames(styles.MenuLayout, "ui secondary pointing menu")}>
        <Logo className={"item"} />
        <Link to={GLOBAL_ROUTES.HOME} className={getTabClasses(GLOBAL_ROUTES.HOME)}>
            Home
        </Link>
        {loggedIn && (
            <>
                <Link to={GLOBAL_ROUTES.PROBLEMS} className={getTabClasses(GLOBAL_ROUTES.PROBLEMS)}>
                    Problems
                </Link>
                <Link to={GLOBAL_ROUTES.TEAM} className={getTabClasses(GLOBAL_ROUTES.TEAM)}>
                    Team
                </Link>
                <Dropdown className={getTabClasses(GLOBAL_ROUTES.INDIVIDUAL_LEADERBOARD, GLOBAL_ROUTES.TEAM_LEADERBOARD)} item text='Leaderboard'>
                    <Dropdown.Menu>
                        <Dropdown.Item
                            className={classNames({ active: isTabActive(GLOBAL_ROUTES.INDIVIDUAL_LEADERBOARD) })}
                            onClick={() => history.push(GLOBAL_ROUTES.INDIVIDUAL_LEADERBOARD)}
                        >
                            Individual
                        </Dropdown.Item>
                        <Dropdown.Item
                            className={classNames({ active: isTabActive(GLOBAL_ROUTES.TEAM_LEADERBOARD) })}
                            onClick={() => history.push(GLOBAL_ROUTES.TEAM_LEADERBOARD)}
                        >
                            Team
                        </Dropdown.Item>
                    </Dropdown.Menu>
                </Dropdown>
                <Link to={GLOBAL_ROUTES.API_GUIDE} className={getTabClasses(GLOBAL_ROUTES.API_GUIDE)}>
                    Api Guide
                </Link>
            </>
        )}
        <div className="right menu">
            {loggedIn ?
                <>
                    <span className="ui item">Logged in: {user?.username}</span>
                    <a onClick={onLogout} className="ui item">
                        Logout
                    </a>
                </>
                :
                <Link to={GLOBAL_ROUTES.LOGIN} className={getTabClasses(GLOBAL_ROUTES.LOGIN)}>
                    Login
                </Link>
            }
        </div>
    </div>;
};