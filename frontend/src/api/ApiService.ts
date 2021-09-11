import {AuthorizingResponse} from "../typings/api";
import axiosInstance from "../axios/axios-instance";
import {Endpoints} from "./endpoints";

export const TOKEN_KEY = "token";
export const REFRESH_TOKEN_KEY = "refresh-token";

export abstract class ApiService {

    public static getRefreshToken(): string | null {
        return localStorage.getItem(REFRESH_TOKEN_KEY);
    }

    public static async refreshCredentials(): Promise<AuthorizingResponse> {
        const resp = await axiosInstance.post<AuthorizingResponse>(Endpoints.REFRESH_TOKEN, {}, {
            headers: {
                "X-Refresh-Token": ApiService.getRefreshToken(),
            },
        });

        ApiService.storeCredentials(resp.data);

        return resp.data;
    }

    public static storeCredentials(payload: AuthorizingResponse): void {
        localStorage.setItem(TOKEN_KEY, payload.access_token);
        localStorage.setItem(REFRESH_TOKEN_KEY, payload.refresh_token);
    }

    public static deleteStoredCredentials(): void {
        localStorage.removeItem(TOKEN_KEY);
        localStorage.removeItem(REFRESH_TOKEN_KEY);
    }

    public static hasStoredCredentials(): boolean {
        return !!localStorage.getItem(REFRESH_TOKEN_KEY);
    }

}