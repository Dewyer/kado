import React from "react";
import styles from "./CustomToast.module.scss";

export interface CustomToastProps {
    message: string,
}

export const CustomToast: React.FC<CustomToastProps> = (props) => {
    const { message } = props;
    return <div>
        <span className={styles.Message}>{message}</span>
    </div>;
};