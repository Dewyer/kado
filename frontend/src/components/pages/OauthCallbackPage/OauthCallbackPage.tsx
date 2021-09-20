import React, {useEffect} from "react";
import {useHistory} from "react-router-dom";

export const OauthCallbackPage: React.FC = () => {
    const history = useHistory();

    useEffect(() => {
        window.addEventListener("message", function (event) {
            if (event.data.message === "requestResult" && event.source) {
                const par = new URLSearchParams(history.location.search);
                const code = par.get('code');
                event.source.postMessage({"message": "deliverResult", result: { token: code } });
            }
        });
    }, []);

    return (
        <>
            <div className="ui segment">
                <p />
                <div className="ui active dimmer">
                    <div className="ui loader" />
                </div>
            </div>
        </>
    );
};
