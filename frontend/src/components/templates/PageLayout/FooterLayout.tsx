import React from "react";
import styles from "./PageLayout.module.scss";
import classNames from "classnames";

export const FooterLayout: React.FC = () => {
    return <div className={classNames(styles.FooterLayout)}>
        <div className="ui divider" />
        Footer
    </div>;
};