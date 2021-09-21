import React from "react";
import styles from "./PageLayout.module.scss";
import classNames from "classnames";
import {GLOBAL_ROUTES} from "src/routing/routingConstants";
import {Link} from "react-router-dom";
import {useWindowSize} from "../../../hooks/useWindowSize";

const SMALLER_SIZE_BR = 500;

export const FooterLayout: React.FC = () => {
    const windowSize = useWindowSize();
    const onSmallerScreen = windowSize.width && windowSize.width <= SMALLER_SIZE_BR;

    return <div className={classNames(styles.FooterLayout)}>
        <div className="ui divider" />
        <div className={classNames(styles.FooterLineContainer, { [styles.SmallerSizeFooter]: onSmallerScreen })}>
            <span>SnapsoftChallenge - 2021, for the SCH QPA</span>
            <span><Link target={"_blank"} to={GLOBAL_ROUTES.TERMS_OF_USE_AND_PRIVACY_POLICY}>privacy policy</Link>,</span>
            <span><Link target={"_blank"} to={GLOBAL_ROUTES.REPORT_PROBLEM}>report a problem</Link>,</span>
            <span><a target={"_blank"} href={"https://snapsoft.hu"}>website</a>,</span>
            <span><a target={"_blank"} href={"https://www.linkedin.com/company/snapsoft-ltd./"}>linkedin</a>,</span>
            <span><a target={"_blank"} href={"https://www.instagram.com/snapsoft.hu/"}>instagram</a></span>
        </div>
    </div>;
};