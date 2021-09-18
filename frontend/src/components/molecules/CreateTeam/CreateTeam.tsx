import React from "react";
import styles from "./CreateTeam.module.scss";
import {Field, Form} from "react-final-form";
import classNames from "classnames";
import {FieldTextInput} from "src/components/atoms/FieldTextInput/FieldTextInput";
import {CreateTeamFormData} from "./CreateTeam.types";
import {validateCreateTeamForm} from "./CreateTeam.validation";
import {useCreateTeamMutation} from "../../../api/hooks/teamApiHooks";
import {queryClient} from "../../../api/queryClient";
import {FieldCheckbox} from "../../atoms/FieldCheckbox/FieldCheckbox";

export interface CreateTeamProps {
}

export const CreateTeam: React.FC<CreateTeamProps> = (props) => {
    const createTeamMutation = useCreateTeamMutation();

    const onSubmit = async (values: CreateTeamFormData) => {
        await createTeamMutation.mutateAsync({
            name: values.name,
            participate_in_leaderboards: values.participateInLeaderBoard,
        });

        await queryClient.invalidateQueries("FetchUserTeam");
    };

    return <div className={styles.CreateTeam}>
        <h3 className="ui header">Create a new team:</h3>
        <p>Create a team and invite other users by sending them your join code.</p>
        <Form<CreateTeamFormData>
            initialValues={{
                name: "",
                participateInLeaderBoard: false,
            }}
            onSubmit={onSubmit}
            validate={validateCreateTeamForm}
            render={({ handleSubmit, submitting, valid }) => (
                <form className={classNames("ui form", styles.CreateTeamForm)} onSubmit={handleSubmit}>
                    <div className="field">
                        <label>Team name</label>
                        <Field<string>
                            containerClassName={styles.FormField}
                            name="name"
                            component={FieldTextInput}
                            placeHolder="ex: Labosch"
                            type="text"
                        />
                    </div>
                    <Field<boolean>
                        name="participateInLeaderBoard"
                        component={FieldCheckbox}
                        label={"Show my team on leaderboards"}
                        type="checkbox"
                    />
                    <button
                        className={classNames("ui button primary", styles.FormField, { loading: createTeamMutation.isLoading })}
                        type="submit"
                        disabled={submitting || !valid || createTeamMutation.isLoading}
                    >
                        Create
                    </button>
                </form>
            )}
        />
    </div>;
};