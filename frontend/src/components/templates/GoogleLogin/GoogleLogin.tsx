import React, {useCallback} from "react";
import {GoogleLoginButton, GoogleLoginResponseFull} from "../../molecules/GoogleLoginButton/GoogleLoginButton";
import {toastPopper} from "../../../helpers/toastPopper";
import {AuthorizationResult} from "../../pages/LoginPage/LoginPage.types";

export interface GoogleLoginProps {
    loading?: boolean;
    disabled?: boolean;
    onAuthorization: (result: AuthorizationResult) => void;
}

export const GoogleLogin: React.FC<GoogleLoginProps> = (props) => {
    const { loading, disabled, onAuthorization } = props;

    const onGoogleLoginSuccessCallback = useCallback(async (response: GoogleLoginResponseFull) => {
        if ('code' in response) {
            toastPopper({ message:"You are offline and can't register!" });
            return;
        }

        onAuthorization({
            authorizer: 'google',
            token: response.tokenId,
        });

    }, [onAuthorization]);

    return (
        <div>
            <GoogleLoginButton
                onSuccess={onGoogleLoginSuccessCallback}
                disabled={loading || !!disabled}
                loading={loading}
            />
        </div>
    );
};
