import React from "react";
import styles from "./Logo.module.scss";
import classNames from "classnames";

export interface LogoProps {
    className?: string;
    small?: boolean;
}

export const Logo: React.FC<LogoProps> = (props) => {
    return <div className={classNames(styles.Logo, props.className, { [styles.small]: props.small })}>
        <span>Snap</span> Challenge
    </div>;
};