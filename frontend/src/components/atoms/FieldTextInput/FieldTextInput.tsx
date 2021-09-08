import React from "react";
import classNames from "classnames";
import {FieldInputProps, FieldMetaState} from "react-final-form";
import {FieldError} from "../FieldError/FieldError";
import styles from "./FieldTextInput.module.scss";

export type TextInputProps = {
    containerClassName?: string;
    input: FieldInputProps<string, HTMLElement>,
    placeHolder?: string;
    meta: FieldMetaState<string>;
};

export const FieldTextInput: React.FC<TextInputProps> = (props) => {
    const {
        containerClassName,
        placeHolder,
        input,
        meta,
    } = props;

    return (
        <div className={classNames("ui input", styles.FieldTextInput, containerClassName)}>
            <input placeholder={placeHolder} {...input} />
            <FieldError meta={meta} />
        </div>
    );
};