import React, {useCallback, useState} from "react";
import styles from "./LoginPage.module.scss";
import { PageLayout } from "src/components/templates/PageLayout/PageLayout";
import {CompleteProfileForm} from "../../templates/CompleteProfileForm/CompleteProfileForm";
import { useHistory } from "react-router-dom";
import {GLOBAL_ROUTES} from "../../../routing/routingConstants";
import {useDispatch, useSelector} from "react-redux";
import {makeSelectUser} from "../../../store/selectors/global.selectors";
import {useCheckUserMutation, useLoginUserMutation} from "../../../api/hooks/authApiHooks";
import {authorizedAction} from "../../../store/actions/global";
import {AuthorizationResult} from "./LoginPage.types";
import { GoogleLogin } from "src/components/templates/GoogleLogin/GoogleLogin";
import {GithubLogin} from "../../templates/GithubLogin/GithubLogin";
import {CheckUserResponse} from "../../../typings/api";
import {toastPopper} from "../../../helpers/toastPopper";

export const LoginPage: React.FC = () => {
    const history = useHistory();
    const dispatch = useDispatch();
    const loggedIn = !!useSelector(makeSelectUser());
    const [lastAuthResult, setLastAuthResult] = useState<AuthorizationResult>();

    const checkUserMutation = useCheckUserMutation();
    const loginUserMutation = useLoginUserMutation();

    const handleExistingUser = useCallback(async (result: AuthorizationResult, userCheckResponse: CheckUserResponse) => {
        if (userCheckResponse.uses_different_authenticator) {
            toastPopper({ message: "There is an existing account with this e-mail address that uses a different login method." });
            return;
        }

        const authorizingResponse = await loginUserMutation.mutateAsync(result);

        dispatch(authorizedAction(authorizingResponse));
        history.push(GLOBAL_ROUTES.HOME);
    }, [history, dispatch, loginUserMutation]);

    const onAuthorizationResult = useCallback(async (result: AuthorizationResult) => {
        const userCheckResponse = await checkUserMutation.mutateAsync(result);

        if (userCheckResponse.user_exists) {
            await handleExistingUser(result, userCheckResponse);
            return;
        }

        setLastAuthResult(result);
    }, [setLastAuthResult, checkUserMutation, history, handleExistingUser]);

    const loading = checkUserMutation.isLoading || loginUserMutation.isLoading;

    return (
        <PageLayout contentClassName={styles.LoginPage}>
            {loggedIn && <span>You are already logged in, what do you want?</span>}
            {!loggedIn && <>
                <span>To participate please log in with google:</span>
                <div className={styles.LoginButtonsWrapper}>
                    <GoogleLogin
                        loading={loading}
                        disabled={!!lastAuthResult}
                        onAuthorization={onAuthorizationResult}
                    />
                    <GithubLogin
                        loading={loading}
                        disabled={!!lastAuthResult}
                        onAuthorization={onAuthorizationResult}
                    />
                </div>

                {lastAuthResult && <CompleteProfileForm
                    loginResponse={lastAuthResult}
                    containerClassName={styles.CompleteProfileForm}
                />}
            </>}
        </PageLayout>
    );
};
