import {useMutation, UseMutationResult} from "react-query";
import {
    AuthorizingResponse,
    CheckUserRequest,
    CheckUserResponse, ExchangeGithubCodeRequest,
    ExchangeGithubCodeResponse,
    LoginRequest,
    RegisterRequest
} from "../../typings/api";
import {toastPopper} from "../../helpers/toastPopper";
import {AuthApiService} from "../apis/AuthApiService";

export const useRegisterUserMutation = (): UseMutationResult<AuthorizingResponse, unknown, RegisterRequest> => {
    return useMutation(
        ["RegisterUserMut"],
        async (request: RegisterRequest) => AuthApiService.registerUser(request),
        {
            onError: () => { toastPopper({ message: "Registering user failed!" }) },
        }
    );
};

export const useLoginUserMutation = (): UseMutationResult<AuthorizingResponse, unknown, LoginRequest> => {
    return useMutation(
        ["LoginUserMutation"],
        async (request: LoginRequest) => AuthApiService.loginUser(request),
        {
            onError: () => { toastPopper({ message: "Logging in failed!" }) },
        }
    );
};

export const useCheckUserMutation = (): UseMutationResult<CheckUserResponse, unknown, CheckUserRequest> => {
    return useMutation(
        ["CheckUserMutation"],
        async (request: CheckUserRequest) => AuthApiService.checkUser(request),
        {
            onError: () => { toastPopper({ message: "Logging in failed!" }) },
        }
    );
};

export const useExchangeGithubCodeMutation = (): UseMutationResult<ExchangeGithubCodeResponse, unknown, ExchangeGithubCodeRequest> => {
    return useMutation(
        ["ExchangeGithubCodeMutation"],
        async (request: ExchangeGithubCodeRequest) => AuthApiService.exchangeGithubCode(request),
        {
            onError: () => { toastPopper({ message: "Github login failed!" }) },
        }
    );
};
