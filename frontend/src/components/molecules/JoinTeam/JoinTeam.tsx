import React from "react";
import styles from "./JoinTeam.module.scss";
import classNames from "classnames";
import {Field, Form} from "react-final-form";
import {FieldTextInput} from "../../atoms/FieldTextInput/FieldTextInput";
import {ValidationErrors} from "final-form";

export interface JoinTeamFormData {
    joinCode: string;
}

const validateJoinTeamForm = (values: JoinTeamFormData): ValidationErrors => {
    const errors: ValidationErrors = {};

    if (!values.joinCode) {
        errors.joinCode = "Required!";
    }

    if (!!values.joinCode && values.joinCode.length <= 8) {
        errors.joinCode = "Must be at least 8 characters!";
    }

    return errors;
}

export const JoinTeam: React.FC = () => {
    const onSubmit = async (values: JoinTeamFormData) => {

    };

    return (
        <div className={styles.JoinTeam}>
            <h3 className="ui header">Join a team using a join code:</h3>
            <p>The join code for your team can be obtained from the team's owner.</p>
            <Form<JoinTeamFormData>
                initialValues={{
                    joinCode: "",
                }}
                onSubmit={onSubmit}
                validate={validateJoinTeamForm}
                render={({ handleSubmit, submitting, valid }) => (
                    <form className={classNames("ui form", styles.JoinTeamForm)} onSubmit={handleSubmit}>
                        <div className="field">
                            <label>Join Code</label>
                            <Field<string>
                                name="joinCode"
                                component={FieldTextInput}
                                placeHolder="ex: 1234efef"
                                type="text"
                            />
                        </div>
                        <button
                            className="ui button primary"
                            type="submit"
                            disabled={submitting || !valid}
                        >
                            Join
                        </button>
                    </form>
                )}
            />
        </div>
    );
};
