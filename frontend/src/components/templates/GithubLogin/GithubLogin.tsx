import React, {useCallback} from "react";
import {toastPopper} from "../../../helpers/toastPopper";
import {AuthorizationResult} from "../../pages/LoginPage/LoginPage.types";
import OAuth2Login from "react-simple-oauth2-login";
import styles from "./GithubLogin.module.scss";
import classNames from "classnames";

export interface GithubLoginProps {
    loading?: boolean;
    disabled?: boolean;
    onAuthorization: (result: AuthorizationResult) => void;
}

export const GithubLogin: React.FC<GithubLoginProps> = (props) => {
    const { loading, disabled, onAuthorization } = props;

    const onSuccess = useCallback(async (response: { token?: string }) => {
        if (disabled) {
            return;
        }

        if (!response.token) {
            toastPopper({ message:"We couldn't log you in with github!" });
            return;
        }

        onAuthorization({
            authorizer: 'github',
            token: response.token,
        });

    }, [onAuthorization, disabled]);

    return (
        <div className={styles.GithubButtonWrapper}>
            <OAuth2Login
                buttonText={"Login with github"}
                className={classNames("ui secondary button", { loading, disabled })}
                authorizationUrl="https://github.com/login/oauth/authorize"
                responseType="token"
                clientId={process.env.REACT_APP_GITHUB_CLIENT_ID}
                redirectUri={process.env.REACT_APP_GITHUB_REDIRECT}
                onSuccess={onSuccess}
                onFailure={onSuccess}
                isCrossOrigin={true}
            />
        </div>
    );
};
