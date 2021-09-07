import React from "react";
import styles from "./PageLayout.module.scss";
import { MenuLayout } from "./MenuLayout";
import {FooterLayout} from "./FooterLayout";

export const PageLayout: React.FC = (props) => {
    const { children } = props;
    return <div className={styles.PageLayout}>
        <MenuLayout />
        <div className={styles.ContentLayout}>
            {children}
        </div>
        <FooterLayout />
    </div>;
};