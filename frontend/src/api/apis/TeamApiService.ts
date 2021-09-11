import {GetUsersTeamResponse} from "../../typings/api";
import axiosInstance from "src/axios/axios-instance";
import {Endpoints} from "src/api/endpoints";

export abstract class TeamApiService {
    public static async getTeam(): Promise<GetUsersTeamResponse> {
        const resp = await axiosInstance.post<GetUsersTeamResponse>(Endpoints.GET_TEAM, {});
        return resp.data;
    }
}