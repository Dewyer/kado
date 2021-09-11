import {useMutation, UseMutationResult} from "react-query";
import {AuthorizingResponse, RegisterRequest} from "../typings/api";
import {ApiService} from "./ApiService";
import {toastPopper} from "../helpers/toastPopper";

export const useRegisterUserMutation = (): UseMutationResult<AuthorizingResponse, unknown, RegisterRequest> => {
    return useMutation(
        ["RegisterUserMut"],
        async (request: RegisterRequest) => ApiService.registerUser(request),
        {
            onError: () => { toastPopper({ message: "Registering user failed!" }) },
        }
    );
};