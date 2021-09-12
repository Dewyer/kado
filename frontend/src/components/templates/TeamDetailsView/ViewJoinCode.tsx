import React from "react";
import {TeamFullyPopulatedDto} from "src/typings/api";
import styles from "./TeamDetailsView.module.scss";
import {CustomTextInput} from "../../atoms/CustomTextInput/CustomTextInput";
import {CopyToClipboard} from "react-copy-to-clipboard";
import {toastPopper} from "../../../helpers/toastPopper";

export type FieldErrorProps = {
    team: TeamFullyPopulatedDto;
};

export const ViewJoinCode: React.FC<FieldErrorProps> = (props) => {
    const {
        team,
    } = props;

    const code = team.join_code || "code unavailable";
    return (
        <div className={styles.ViewJoinCode}>
            <label>Join code: </label>
            <CustomTextInput
                readOnly
                value={code}
            />
            <CopyToClipboard
                text={code}
                onCopy={() => toastPopper({ message: "Code copied to clipboard." })}
            >
                <button className="ui button">
                    Copy
                </button>
            </CopyToClipboard>
        </div>
    );
};