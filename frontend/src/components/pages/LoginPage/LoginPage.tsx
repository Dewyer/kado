import React from "react";
import styles from "./LoginPage.module.scss";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {GoogleLoginButton} from "../../molecules/GoogleLoginButton/GoogleLoginButton";

export const LoginPage: React.FC = () => {
    return (
        <PageLayout contentClassName={styles.LoginPage}>
            <span>To participate please log in with google:</span>
            <GoogleLoginButton />
        </PageLayout>
    );
};
