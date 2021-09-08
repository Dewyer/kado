import React, {useCallback, useState} from "react";
import styles from "./LoginPage.module.scss";
import { PageLayout } from "src/components/templates/PageLayout/PageLayout";
import {GoogleLoginButton, GoogleLoginResponseFull} from "../../molecules/GoogleLoginButton/GoogleLoginButton";
import {CompleteProfileForm} from "./CompleteProfileForm/CompleteProfileForm";
import {ApiService} from "src/api/ApiService";
import { useHistory } from "react-router-dom";
import {GLOBAL_ROUTES} from "../../../routing/routingConstants";

// 1. google login -> token
// 2. kell egy api check hívás -> ha valid a google token akkor megmondja h van e fiokunk

export const LoginPage: React.FC = () => {
    const history = useHistory();
    const [lastGoogleLoginResult, setLastGoogleLoginResult] = useState<GoogleLoginResponseFull>({ tokenId: '' } as unknown as GoogleLoginResponseFull);
    const [checkLoading, setCheckLoading] = useState(false);

    const onGoogleLoginSuccessCallback = useCallback(async (response: GoogleLoginResponseFull) => {
        if ('code' in response) {
            return;
        }

        setCheckLoading(true);
        const userCheckResponse = await ApiService.checkUser({
            authorizer: 'google',
            token: response.tokenId,
        });

        // TODO if logged in do some stuff
        if (userCheckResponse.user_exists) {
            history.push(GLOBAL_ROUTES.HOME);
            return;
        }

        setLastGoogleLoginResult(response);
        setCheckLoading(false);
    }, [setLastGoogleLoginResult]);

    return (
        <PageLayout contentClassName={styles.LoginPage}>
            <span>To participate please log in with google:</span>
            <GoogleLoginButton
                onSuccess={onGoogleLoginSuccessCallback}
                disabled={checkLoading || !!lastGoogleLoginResult}
            />

            {lastGoogleLoginResult && <CompleteProfileForm
                loginResponse={lastGoogleLoginResult}
                containerClassName={styles.CompleteProfileForm}
            />}
        </PageLayout>
    );
};
