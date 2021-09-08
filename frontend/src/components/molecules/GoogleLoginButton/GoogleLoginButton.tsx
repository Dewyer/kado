import React, {useCallback} from "react";
import classNames from "classnames";
import styles from "./GoogleLoginButton.module.scss";
import GoogleLogin, {GoogleLoginResponse, GoogleLoginResponseOffline} from "react-google-login";

export type GoogleLoginError = unknown;
export type GoogleLoginResponseFull = GoogleLoginResponse | GoogleLoginResponseOffline;

export interface GoogleLoginButtonProps {
    onSuccess?: (response: GoogleLoginResponseFull) => void;
    onFailure?: (error: GoogleLoginError) => void;
    disabled?: boolean;
}

export const GoogleLoginButton: React.FC<GoogleLoginButtonProps> = (props) => {
    const { onSuccess, onFailure, disabled } = props;

    const onSuccessCallback = useCallback((response: GoogleLoginResponseFull) => {
        if (onSuccess) {
            onSuccess(response);
        }
    }, [onSuccess]);

    const onFailureCallback = useCallback((error: GoogleLoginError) => {
        if (onFailure) {
            onFailure(error);
        }
    }, [onFailure]);

    return <GoogleLogin
        clientId={process.env.REACT_APP_GOOGLE_CLIENT_ID || ''}
        render={renderProps => (
            <button onClick={renderProps.onClick} disabled={renderProps.disabled || disabled} className={classNames("ui primary button", styles.GoogleLoginButton)}>
                Login with google
            </button>
        )}
        buttonText="Login with google"
        onSuccess={onSuccessCallback}
        onFailure={onFailureCallback}
        cookiePolicy={'single_host_origin'}
    />;
};