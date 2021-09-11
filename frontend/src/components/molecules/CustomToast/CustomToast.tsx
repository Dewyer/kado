import React from "react";

export interface CustomToastProps {
    message: string,
}

export const CustomToast: React.FC<CustomToastProps> = (props) => {
    const { message } = props;
    return <div>
        <span>{message}</span>
    </div>;
};