import React from "react";
import {GoogleLoginResponseFull} from "src/components/molecules/GoogleLoginButton/GoogleLoginButton";
import styles from "./CompleteProfileForm.module.scss";
import { Form, Field } from 'react-final-form'
import {CompleteProfileFormData} from "./CompleteProfileForm.types";
import {completeProfileFormValidation} from "./CompleteProfileForm.validation";
import {FieldTextInput} from "src/components/atoms/FieldTextInput/FieldTextInput";
import { Checkbox } from "semantic-ui-react";
import {FieldCheckbox} from "../../../atoms/FieldCheckbox/FieldCheckbox";
import classNames from "classnames";

export interface CompleteProfileFormProps {
    containerClassName?: string;
    loginResponse: GoogleLoginResponseFull,
}

export const CompleteProfileForm: React.FC<CompleteProfileFormProps> = (props) => {
    const { loginResponse, containerClassName } = props;

    const onSubmit = (values: CompleteProfileFormData) => {
        console.log(values, loginResponse);
    };

    return (
        <div className={classNames(styles.CompleteProfileForm, containerClassName)}>
            <h2 className="ui header">Register</h2>
            <p>A user with this email address doesn't exist. To register please fill out the following form:</p>

            <Form<CompleteProfileFormData>
                initialValues={{
                    username: "",
                    participateInLeaderBoard: true,
                }}
                onSubmit={onSubmit}
                validate={completeProfileFormValidation}
                render={({ handleSubmit, submitting, valid }) => (
                    <form className={styles.Form} onSubmit={handleSubmit}>
                        <Field<string>
                            name="username"
                            component={FieldTextInput}
                            placeHolder="Username"
                            type="text"
                        />
                        <Field<boolean>
                            name="participateInLeaderBoard"
                            component={FieldCheckbox}
                            label={"Show me on leaderboards"}
                            type="checkbox"
                        />

                        <button className="ui primary button" type="submit" disabled={submitting || !valid}>
                            Register
                        </button>
                    </form>
                )}
            />
        </div>
    );
};