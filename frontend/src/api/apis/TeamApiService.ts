import {
    CreateTeamRequest,
    CreateTeamResponse,
    GetUsersTeamResponse,
    JoinTeamRequest,
    LeaveTeamRequest
} from "src/typings/api";
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
            data: payload,
        });

        return resp.data;
    }

    public static async leaveTeam(payload: LeaveTeamRequest): Promise<void> {
        await ApiService.authenticatedRequest<CreateTeamResponse>({
            url: Endpoints.LEAVE_TEAM,
            method: "POST",
            data: payload,
        });
    }

    public static async joinTeam(payload: JoinTeamRequest): Promise<void> {
        await ApiService.authenticatedRequest<void>({
            url: Endpoints.JOIN_TEAM,
            method: "POST",
            data: payload,
        });
    }
}