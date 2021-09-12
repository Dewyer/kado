import {ValidationErrors} from "final-form";
import {CreateTeamFormData} from "./CreateTeam.types";

export const validateCreateTeamForm = (values: CreateTeamFormData): ValidationErrors => {
    const errors: ValidationErrors = {};

    if (!values.name) {
        errors.name = "Required!";
    }

    if (!!values.name && values.name.length < 3) {
        errors.name = "Must be at least 3 characters!";
    }

    return errors;
}