import React from "react";
import styles from "./PageLayout.module.scss";
import classNames from "classnames";

export const FooterLayout: React.FC = () => {
    return <div className={classNames(styles.FooterLayout)}>
        <div className="ui divider" />
        <span>
            SnapsoftChallenge - 2021, for the SCH QPA{" "}
            <a target={"_blank"} href={"https://snapsoft.hu"}>website</a>,{" "}
            <a target={"_blank"} href={"https://www.linkedin.com/company/snapsoft-ltd./"}>linkedin</a>,{" "}
            <a target={"_blank"} href={"https://www.instagram.com/snapsoft.hu/"}>instagram</a>
        </span>
    </div>;
};