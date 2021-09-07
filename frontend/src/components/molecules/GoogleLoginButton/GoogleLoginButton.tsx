import React from "react";
import classNames from "classnames";
import styles from "./GoogleLoginButton.module.scss";
import GoogleLogin from "react-google-login";

export const GoogleLoginButton: React.FC = () => {
    const responseGoogle = (response: any) => {
        console.log(response, "google response");
    }

    return <GoogleLogin
        clientId={process.env.REACT_APP_GOOGLE_CLIENT_ID || ''}
        render={renderProps => (
            <button onClick={renderProps.onClick} disabled={renderProps.disabled} className={classNames("ui primary button", styles.GoogleLoginButton)}>
                Login with google
            </button>
        )}
        buttonText="Login with google"
        onSuccess={responseGoogle}
        onFailure={responseGoogle}
        cookiePolicy={'single_host_origin'}
    />;
};