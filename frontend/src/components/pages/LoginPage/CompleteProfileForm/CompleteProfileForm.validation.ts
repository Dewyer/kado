import {CompleteProfileFormData} from "./CompleteProfileForm.types";
import {ValidationErrors} from "final-form";

export const completeProfileFormValidation = (values: CompleteProfileFormData): ValidationErrors => {
    const errors: ValidationErrors = {};

    if (!values.username) {
        errors.username = "Required";
    }

    return errors;
};