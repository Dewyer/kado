import React from "react";
import styles from "./PageLayout.module.scss";
import { MenuLayout } from "./MenuLayout";
import { FooterLayout } from "./FooterLayout";
import classNames from "classnames";

export interface PageLayoutProps {
    contentClassName?: string;
}

export const PageLayout: React.FC<PageLayoutProps> = (props) => {
    const { children } = props;
    return <div className={styles.PageLayout}>
        <MenuLayout />
        <div className={classNames(styles.ContentLayout, props.contentClassName)}>
            {children}
        </div>
        <FooterLayout />
    </div>;
};