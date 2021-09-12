import React from "react";
import styles from "./ViewAndRefreshApiToken.module.scss";
import {CustomTextInput} from "src/components/atoms/CustomTextInput/CustomTextInput";
import {CopyToClipboard} from "react-copy-to-clipboard";
import {toastPopper} from "src/helpers/toastPopper";
import {useFetchApiToken, useRefreshApiTokenMutation} from "src/api/hooks/apiTokenApiHooks";
import {PageLoader} from "../../templates/PageLoader/PageLoader";
import {queryClient} from "../../../api/queryClient";

export const ViewAndRefreshApiToken: React.FC = () => {
    const apiTokenQuery = useFetchApiToken();
    const refreshApiTokenMutation = useRefreshApiTokenMutation();

    if (apiTokenQuery.isLoading || !apiTokenQuery.data) {
        return (
            <PageLoader message={"Api token loading ..."} />
        );
    }

    const onRefreshApiToken = async () => {
        await refreshApiTokenMutation.mutateAsync();
        await queryClient.invalidateQueries("FetchApiToken");

        toastPopper({ message: "Token refreshed!" });
    };

    const token = apiTokenQuery.data.token.token;

    return (
        <div className={styles.ViewAndRefreshApiToken}>
            <h4 className={"ui header"}>Your api token: </h4>
            <div className={styles.ControlRow}>
                <CustomTextInput
                    type={"password"}
                    readOnly
                    value={token}
                />

                <button
                    className="ui button primary"
                    disabled={refreshApiTokenMutation.isLoading}
                    onClick={onRefreshApiToken}
                >
                    Refresh
                </button>

                <CopyToClipboard
                    text={token}
                    onCopy={() => toastPopper({ message: "Token copied to clipboard." })}
                >
                    <button className="ui button">
                        Copy
                    </button>
                </CopyToClipboard>
            </div>
        </div>
    );
};
