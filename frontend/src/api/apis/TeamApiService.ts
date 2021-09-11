import {CreateTeamRequest, CreateTeamResponse, GetUsersTeamResponse} from "src/typings/api";
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

    public static async createTeam(payload: CreateTeamRequest): Promise<CreateTeamResponse> {
        const resp = await ApiService.authenticatedRequest<CreateTeamResponse>({
            url: Endpoints.CREATE_TEAM,
            method: "POST",
        });

        return resp.data;
    }
}