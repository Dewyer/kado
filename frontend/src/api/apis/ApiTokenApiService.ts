import {
    GetApiTokenResponse,
    GetProblemDetailsResponse,
    GetProblemsResponse,
} from "src/typings/api";
import {Endpoints} from "src/api/endpoints";
import {ApiService} from "../ApiService";

export abstract class ApiTokenApiService {
    public static async getApiToken(): Promise<GetApiTokenResponse> {
        const resp = await ApiService.authenticatedRequest<GetApiTokenResponse>({
            url: Endpoints.GET_API_TOKEN,
            method: "GET",
        });

        return resp.data;
    }

    public static async refreshApiToken(): Promise<GetApiTokenResponse> {
        const resp = await ApiService.authenticatedRequest<GetApiTokenResponse>({
            url: Endpoints.REFRESH_API_TOKEN,
            method: "POST",
        });

        return resp.data;
    }
}