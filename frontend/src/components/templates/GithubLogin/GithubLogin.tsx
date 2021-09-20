import React, {useCallback} from "react";
import {toastPopper} from "../../../helpers/toastPopper";
import {AuthorizationResult} from "../../pages/LoginPage/LoginPage.types";
import OAuth2Login from 'react-simple-oauth2-login';

export interface GithubLoginProps {
    loading?: boolean;
    disabled?: boolean;
    onAuthorization: (result: AuthorizationResult) => void;
}

export const GithubLogin: React.FC<GithubLoginProps> = (props) => {
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
        <>
            <OAuth2Login
                authorizationUrl="https://accounts.spotify.com/authorize"
                responseType="token"
                clientId="9822046hvr4lnhi7g07grihpefahy5jb"
                redirectUri="http://localhost:3000/oauth-callback"
                onSuccess={onSuccess}
                onFailure={onFailure}/>
        </>
    );
};
