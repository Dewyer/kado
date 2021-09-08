import axiosInstance from "../axios/axios-instance";
import {Endpoints} from "./endpoints";
import {CheckUserRequest, CheckUserResponse} from "../typings/api";

export abstract class ApiService {

    public static async checkUser(payload: CheckUserRequest): Promise<CheckUserResponse> {
        const resp = await axiosInstance.post<CheckUserResponse>(Endpoints.CHECK_USER, payload);
        return resp.data;
    }
}