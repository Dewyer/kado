import React from "react";
import styles from "./PageLayout.module.scss";
import classNames from "classnames";
import {GLOBAL_ROUTES} from "src/routing/routingConstants";
import {Link} from "react-router-dom";

export const FooterLayout: React.FC = () => {
    return <div className={classNames(styles.FooterLayout)}>
        <div className="ui divider" />
        <span>
            SnapsoftChallenge - 2021, for the SCH QPA{" "}
            <Link target={"_blank"} to={GLOBAL_ROUTES.TERMS_OF_USE_AND_PRIVACY_POLICY}>privacy policy</Link>,{" "}
            <Link target={"_blank"} to={GLOBAL_ROUTES.REPORT_PROBLEM}>report a problem</Link>,{" "}
            <a target={"_blank"} href={"https://snapsoft.hu"}>website</a>,{" "}
            <a target={"_blank"} href={"https://www.linkedin.com/company/snapsoft-ltd./"}>linkedin</a>,{" "}
            <a target={"_blank"} href={"https://www.instagram.com/snapsoft.hu/"}>instagram</a>
        </span>
    </div>;
};