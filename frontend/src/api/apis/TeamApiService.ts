import {GetUsersTeamResponse} from "../../typings/api";
import axiosInstance from "src/axios/axios-instance";
import {Endpoints} from "src/api/endpoints";
import {ApiService} from "../ApiService";

export abstract class TeamApiService {
    public static async getTeam(): Promise<GetUsersTeamResponse> {
        const resp = await ApiService.authenticatedRequest<GetUsersTeamResponse>({
            url: Endpoints.GET_TEAM,
            method: "GET",
        });

        return resp.data;
    }
}