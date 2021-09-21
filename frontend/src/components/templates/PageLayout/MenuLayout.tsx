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
import {useWindowSize} from "src/hooks/useWindowSize";
import {UserDto} from "../../../typings/api";
import { History } from "history";

const SMALLER_SIZE_BR = 1000;

export interface NormalMenuProps {
    loggedIn: boolean,
    user?: UserDto,
    onLogout: () => void,
}

const isTabActive = (route: string, history: History) => {
    return history.location.pathname === route;
};

const getTabClasses = (history: History, route: string, routeTwo?: string) => {
    return classNames("item", { active: isTabActive(route, history) || (routeTwo ? isTabActive(routeTwo, history) : false) });
};

const RightLoginMenu: React.FC<{
    user?: UserDto,
    onLogout: () => void,
}> = (props) => {
    const history = useHistory();
    const windowSize = useWindowSize();
    const onSmallerScreen = windowSize.width && windowSize.width <= SMALLER_SIZE_BR;

    return (
        <div className="right menu">
            {props.children}
            {!!props.user ?
                <>
                    <span className={classNames("ui item", { [styles.SmallUsername]: onSmallerScreen })} >{onSmallerScreen ? "" : "Logged in:"} {props.user?.username}</span>
                    <a onClick={props.onLogout} className="ui item">
                        Logout
                    </a>
                </>
                :
                <Link to={GLOBAL_ROUTES.LOGIN} className={getTabClasses(history, GLOBAL_ROUTES.LOGIN)}>
                    Login
                </Link>
            }
        </div>
    );
};

const NormalMenu: React.FC<NormalMenuProps> = (props) => {
    const { loggedIn, user, onLogout } = props;
    const history = useHistory();

    return (
        <>
            <Logo className={"item"} />
            <Link to={GLOBAL_ROUTES.HOME} className={getTabClasses(history, GLOBAL_ROUTES.HOME)}>
                Home
            </Link>
            {loggedIn && (
                <>
                    <Link to={GLOBAL_ROUTES.PROBLEMS} className={getTabClasses(history,GLOBAL_ROUTES.PROBLEMS)}>
                        Problems
                    </Link>
                    <Link to={GLOBAL_ROUTES.TEAM} className={getTabClasses(history,GLOBAL_ROUTES.TEAM)}>
                        Team
                    </Link>
                    <Dropdown className={getTabClasses(history, GLOBAL_ROUTES.INDIVIDUAL_LEADERBOARD, GLOBAL_ROUTES.TEAM_LEADERBOARD)} item text='Leaderboard'>
                        <Dropdown.Menu>
                            <Dropdown.Item
                                className={classNames({ active: isTabActive(GLOBAL_ROUTES.INDIVIDUAL_LEADERBOARD, history) })}
                                onClick={() => history.push(GLOBAL_ROUTES.INDIVIDUAL_LEADERBOARD)}
                            >
                                Individual
                            </Dropdown.Item>
                            <Dropdown.Item
                                className={classNames({ active: isTabActive(GLOBAL_ROUTES.TEAM_LEADERBOARD, history) })}
                                onClick={() => history.push(GLOBAL_ROUTES.TEAM_LEADERBOARD)}
                            >
                                Team
                            </Dropdown.Item>
                        </Dropdown.Menu>
                    </Dropdown>
                    <Link to={GLOBAL_ROUTES.API_GUIDE} className={getTabClasses(history, GLOBAL_ROUTES.API_GUIDE)}>
                        Api Guide
                    </Link>
                </>
            )}
            <RightLoginMenu
                user={user}
                onLogout={onLogout}
            />
        </>
    );
};

export const SmallScreenMenu: React.FC<NormalMenuProps> = (props) => {
    const { loggedIn, user, onLogout } = props;

    const history = useHistory();
    const routes: {
        route: string,
        name: string,
        protectedRoute?: boolean
    }[] = [
        { route: GLOBAL_ROUTES.HOME, name: "Home" },
        { route: GLOBAL_ROUTES.PROBLEMS, name: "Problems", protectedRoute: true },
        { route: GLOBAL_ROUTES.TEAM, name: "Team", protectedRoute: true },
        { route: GLOBAL_ROUTES.INDIVIDUAL_LEADERBOARD, name: "Individual leaderboard", protectedRoute: true },
        { route: GLOBAL_ROUTES.TEAM_LEADERBOARD, name: "Team leaderboard", protectedRoute: true },
        { route: GLOBAL_ROUTES.API_GUIDE, name: "API Guide", protectedRoute: true },
    ];

    return (
        <>
            <Logo className={"item"} small />
            <RightLoginMenu
                user={user}
                onLogout={onLogout}
            >
                <Dropdown item text="☰">
                    <Dropdown.Menu>
                        {routes.map((route) => (
                            ((loggedIn && route.protectedRoute) || !route.protectedRoute) &&
                            <Dropdown.Item
                                key={route.route}
                                className={classNames({ active: isTabActive(route.route, history) })}
                                onClick={() => history.push(route.route)}
                            >
                                {route.name}
                            </Dropdown.Item>
                        ))}
                    </Dropdown.Menu>
                </Dropdown>
            </RightLoginMenu>
        </>
    );
};

export const MenuLayout: React.FC = () => {
    const dispatch = useDispatch();
    const windowSize = useWindowSize();
    const onSmallerScreen = windowSize.width && windowSize.width <= SMALLER_SIZE_BR;

    const user = useSelector(makeSelectUser());
    const loggedIn = !!user;

    const onLogout = () => {
        dispatch(logoutAction());
    };

    return <div className={classNames(styles.MenuLayout, "ui secondary pointing menu", {
        [styles.SmallerSizeMenu]: onSmallerScreen,
    })}>
        {!onSmallerScreen ? <NormalMenu
            loggedIn={loggedIn}
            user={user}
            onLogout={onLogout}
        /> : <SmallScreenMenu
            loggedIn={loggedIn}
            user={user}
            onLogout={onLogout}
        />}
    </div>;
};