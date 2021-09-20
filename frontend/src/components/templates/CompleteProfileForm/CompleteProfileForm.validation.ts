import {CompleteProfileFormData} from "./CompleteProfileForm.types";
import {ValidationErrors} from "final-form";

export const completeProfileFormValidation = (values: CompleteProfileFormData): ValidationErrors => {
    const errors: ValidationErrors = {};

    if (!values.username) {
        errors.username = "Required";
    }

    if (values.username && (values.username?.length || 0) <= 4) {
        errors.username = "Must be at least 4 characters long";
    }

    if (!values.acceptTos) {
        errors.acceptTos = "You must accept this to register (I mean websites have been having stuff like this the 1920s what is your problem)";
    }

    return errors;
};