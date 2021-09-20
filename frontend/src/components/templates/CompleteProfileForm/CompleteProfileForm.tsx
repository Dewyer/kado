import React from "react";
import styles from "./CompleteProfileForm.module.scss";
import { Form, Field } from 'react-final-form'
import {CompleteProfileFormData} from "./CompleteProfileForm.types";
import {completeProfileFormValidation} from "./CompleteProfileForm.validation";
import {FieldTextInput} from "src/components/atoms/FieldTextInput/FieldTextInput";
import {FieldCheckbox} from "src/components/atoms/FieldCheckbox/FieldCheckbox";
import classNames from "classnames";
import {useRegisterUserMutation} from "src/api/hooks/authApiHooks";
import {useDispatch} from "react-redux";
import {authorizedAction} from "src/store/actions/global";
import {history} from "src/helpers/history";
import {GLOBAL_ROUTES} from "src/routing/routingConstants";
import {AuthorizationResult} from "src/components/pages/LoginPage/LoginPage.types";
import { Link } from "react-router-dom";
import {toastPopper} from "../../../helpers/toastPopper";

export interface CompleteProfileFormProps {
    containerClassName?: string;
    loginResponse: AuthorizationResult,
}

export const CompleteProfileForm: React.FC<CompleteProfileFormProps> = (props) => {
    const { loginResponse, containerClassName } = props;
    const dispatch = useDispatch();
    const registerUserMutation = useRegisterUserMutation();

    const onSubmit = async (values: CompleteProfileFormData) => {
        if (values.username === "69420") {
            toastPopper({ message: "Nice" });
            return;
        }

        const authorizingResponse = await registerUserMutation.mutateAsync({
            ...loginResponse,
            username: values.username,
            participate_in_leaderboards: values.participateInLeaderBoard,
        });

        history.push(GLOBAL_ROUTES.HOME);
        dispatch(authorizedAction(authorizingResponse));
    };

    const submitButtonClasses = classNames("ui primary button" , {
        "loading": registerUserMutation.isLoading,
    });

    return (
        <div className={classNames(styles.CompleteProfileForm, containerClassName)}>
            <h2 className="ui header">Register</h2>
            <p>A user with this email address doesn't exist. To register please fill out the following form:</p>

            <Form<CompleteProfileFormData>
                initialValues={{
                    username: "",
                    participateInLeaderBoard: true,
                    acceptTos: false,
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
                        <Field<boolean>
                            name="acceptTos"
                            component={FieldCheckbox}
                            label={<>I accept the <Link target={"_blank"} to={GLOBAL_ROUTES.TERMS_OF_USE_AND_PRIVACY_POLICY}>terms of use, privacy policy</Link> and the <Link target={"_blank"} to={GLOBAL_ROUTES.HOME}>rules</Link> </>}
                            type="checkbox"
                        />

                        <button className={submitButtonClasses} type="submit" disabled={submitting || !valid || registerUserMutation.isLoading}>
                            Register
                        </button>
                    </form>
                )}
            />
        </div>
    );
};