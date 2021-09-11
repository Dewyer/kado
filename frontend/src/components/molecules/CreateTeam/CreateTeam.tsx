import React from "react";
import styles from "./CreateTeam.module.scss";
import {Field, Form} from "react-final-form";
import classNames from "classnames";
import {FieldTextInput} from "src/components/atoms/FieldTextInput/FieldTextInput";
import {CreateTeamFormData} from "./CreateTeam.types";
import {validateCreateTeamForm} from "./CreateTeam.validation";

export interface CreateTeamProps {
}

export const CreateTeam: React.FC<CreateTeamProps> = (props) => {

    const onSubmit = async (values: CreateTeamFormData) => {
        //TODO create team api
    };

    return <div className={styles.CreateTeam}>
        <h3 className="ui header">Create a new team:</h3>
        <p>Create a team and invite other users by sending them your join code.</p>
        <Form<CreateTeamFormData>
            initialValues={{
                name: "",
            }}
            onSubmit={onSubmit}
            validate={validateCreateTeamForm}
            render={({ handleSubmit, submitting, valid }) => (
                <form className={classNames("ui form", styles.CreateTeamForm)} onSubmit={handleSubmit}>
                    <div className="field">
                        <label>Team name</label>
                        <Field<string>
                            name="name"
                            component={FieldTextInput}
                            placeHolder="ex: Labosch"
                            type="text"
                        />
                    </div>
                    <button
                        className="ui button primary"
                        type="submit"
                        disabled={submitting || !valid}
                    >
                        Create
                    </button>
                </form>
            )}
        />
    </div>;
};