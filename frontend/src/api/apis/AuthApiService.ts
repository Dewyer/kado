import {AuthorizingResponse, CheckUserRequest, CheckUserResponse, RegisterRequest, LoginRequest} from "../../typings/api";
import axiosInstance from "../../axios/axios-instance";
import {Endpoints} from "../endpoints";

export abstract class AuthApiService {

    public static async checkUser(payload: CheckUserRequest): Promise<CheckUserResponse> {
        const resp = await axiosInstance.post<CheckUserResponse>(Endpoints.CHECK_USER, payload);
        return resp.data;
    }

    public static async registerUser(payload: RegisterRequest): Promise<AuthorizingResponse> {
        const resp = await axiosInstance.post<AuthorizingResponse>(Endpoints.REGISTER_USER, payload);
        return resp.data;
    }

    public static async loginUser(payload: LoginRequest): Promise<AuthorizingResponse> {
        const resp = await axiosInstance.post<AuthorizingResponse>(Endpoints.LOGIN_USER, payload);
        return resp.data;
    }
}