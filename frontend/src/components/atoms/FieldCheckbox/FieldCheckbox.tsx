import React from "react";
import classNames from "classnames";
import {FieldInputProps, FieldMetaState} from "react-final-form";
import {FieldError} from "../FieldError/FieldError";

export type FieldCheckboxProps = {
    containerClassName?: string;
    input: FieldInputProps<boolean, HTMLElement>,
    label?: string;
    meta: FieldMetaState<boolean>;
};

export const FieldCheckbox: React.FC<FieldCheckboxProps> = (props) => {
    const {
        containerClassName,
        label,
        input,
        meta,
    } = props;

    const {
        checked,
        value,
        ...restInput
    } = input;

    return (
        <div className={classNames("ui checkbox", containerClassName)}>
            <input type="checkbox" checked={checked} {...restInput} />
            <label>{label}</label>
            <FieldError meta={meta} />
        </div>
    );
};