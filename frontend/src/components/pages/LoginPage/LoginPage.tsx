import React, {useCallback, useState} from "react";
import styles from "./LoginPage.module.scss";
import { PageLayout } from "src/components/templates/PageLayout/PageLayout";
import {GoogleLoginButton, GoogleLoginResponseFull} from "../../molecules/GoogleLoginButton/GoogleLoginButton";
import {CompleteProfileForm} from "../../templates/CompleteProfileForm/CompleteProfileForm";
import { useHistory } from "react-router-dom";
import {GLOBAL_ROUTES} from "../../../routing/routingConstants";
import {toastPopper} from "../../../helpers/toastPopper";
import {useDispatch, useSelector} from "react-redux";
import {makeSelectUser} from "../../../store/selectors/global.selectors";
import {useCheckUserMutation, useLoginUserMutation} from "../../../api/hooks/authApiHooks";
import {GoogleLoginResponse} from "react-google-login";
import {AuthApiService} from "../../../api/apis/AuthApiService";
import {authorizedAction} from "../../../store/actions/global";

export const LoginPage: React.FC = () => {
    const history = useHistory();
    const dispatch = useDispatch();
    const loggedIn = !!useSelector(makeSelectUser());
    const [lastGoogleLoginResult, setLastGoogleLoginResult] = useState<GoogleLoginResponseFull>();

    const checkUserMutation = useCheckUserMutation();
    const loginUserMutation = useLoginUserMutation();

    const handleExistingUser = async (response: GoogleLoginResponse) => {
        const authorizingResponse = await loginUserMutation.mutateAsync({
            authorizer: 'google',
            token: response.tokenId,
        });

        dispatch(authorizedAction(authorizingResponse));
        history.push(GLOBAL_ROUTES.HOME);
    };

    const onGoogleLoginSuccessCallback = useCallback(async (response: GoogleLoginResponseFull) => {
        if ('code' in response) {
            toastPopper({ message:"You are offline and can't register!" });
            history.push(GLOBAL_ROUTES.HOME);
            return;
        }

        const userCheckResponse = await checkUserMutation.mutateAsync({
            authorizer: 'google',
            token: response.tokenId,
        });

        if (userCheckResponse.user_exists) {
            await handleExistingUser(response);
            return;
        }

        setLastGoogleLoginResult(response);
    }, [setLastGoogleLoginResult]);
    const googleLoginLoading = checkUserMutation.isLoading || loginUserMutation.isLoading;

    return (
        <PageLayout contentClassName={styles.LoginPage}>
            {loggedIn && <span>You are already logged in, what do you want?</span>}
            {!loggedIn && <>
                <span>To participate please log in with google:</span>
                <GoogleLoginButton
                    onSuccess={onGoogleLoginSuccessCallback}
                    disabled={googleLoginLoading || !!lastGoogleLoginResult}
                    loading={googleLoginLoading}
                />

                {lastGoogleLoginResult && <CompleteProfileForm
                    loginResponse={lastGoogleLoginResult}
                    containerClassName={styles.CompleteProfileForm}
                />}
            </>}
        </PageLayout>
    );
};
