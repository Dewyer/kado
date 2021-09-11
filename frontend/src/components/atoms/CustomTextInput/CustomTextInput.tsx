import React from "react";
import classNames from "classnames";
import {FieldInputProps, FieldMetaState} from "react-final-form";
import {FieldError} from "../FieldError/FieldError";
import styles from "./FieldTextInput.module.scss";

export type CustomTextInputProps  = {
    containerClassName?: string;
} & React.DetailedHTMLProps<React.InputHTMLAttributes<HTMLInputElement>, HTMLInputElement>;

export const CustomTextInput: React.FC<CustomTextInputProps> = (props) => {
    const {
        containerClassName,
        ...rest
    } = props;

    return (
        <div className={classNames("ui input", containerClassName)}>
            <input type={"text"} {...rest} />
        </div>
    );
};