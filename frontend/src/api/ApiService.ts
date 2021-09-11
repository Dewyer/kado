import axiosInstance from "../axios/axios-instance";
import {Endpoints} from "./endpoints";
import {AuthorizingResponse, CheckUserRequest, CheckUserResponse, RegisterRequest} from "../typings/api";

const TOKEN_KEY = "token";
const REFRESH_TOKEN_KEY = "refresh-token";

export abstract class ApiService {

    public static storeCredentials(payload: AuthorizingResponse): void {
        localStorage.setItem(TOKEN_KEY, payload.access_token);
        localStorage.setItem(REFRESH_TOKEN_KEY, payload.refresh_token);
    }

    public static async checkUser(payload: CheckUserRequest): Promise<CheckUserResponse> {
        const resp = await axiosInstance.post<CheckUserResponse>(Endpoints.CHECK_USER, payload);
        return resp.data;
    }

    public static async registerUser(payload: RegisterRequest): Promise<AuthorizingResponse> {
        const resp = await axiosInstance.post<AuthorizingResponse>(Endpoints.REGISTER_USER, payload);
        return resp.data;
    }
}