import React from "react";
import {FieldMetaState} from "react-final-form";

export type FieldErrorProps = {
    meta: FieldMetaState<unknown>;
};

export const FieldError: React.FC<FieldErrorProps> = (props) => {
    const {
        meta,
    } = props;

    return (
        <>
            {meta.touched && !!meta.error &&
                <div className="field">
                    <div
                        className="ui pointing red basic label">
                            {meta.error}
                    </div>
                </div>
            }
        </>
    );
};