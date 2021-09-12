import styles from "./PageLoader.module.scss";
import React from "react";

export interface PageLoaderProps {
    message: string;
}

export const PageLoader: React.FC<PageLoaderProps> = (props) => (
    <div className={styles.PageLoader}>
        <div className="ui active inline loader"></div>
        <div className="ui text">{props.message}</div>
    </div>
);