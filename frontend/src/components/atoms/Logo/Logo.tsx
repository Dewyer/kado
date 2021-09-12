import React from "react";
import styles from "./Logo.module.scss";
import classNames from "classnames";

export interface LogoProps {
    className?: string;
}

export const Logo: React.FC<LogoProps> = (props) => {
    return <div className={classNames(styles.Logo, props.className)}>
        <span>Snap</span> Problems
    </div>;
};