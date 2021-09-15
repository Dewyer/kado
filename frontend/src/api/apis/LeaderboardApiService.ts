import {
    GetIndividualLeaderboardResponse,
} from "src/typings/api";
import {Endpoints} from "src/api/endpoints";
import {ApiService} from "../ApiService";
import {PaginationOptions} from "src/typings/commonTypes";

export abstract class LeaderboardApiService {
    public static async getIndividualLeaderboard(pagination: PaginationOptions): Promise<GetIndividualLeaderboardResponse> {
        const resp = await ApiService.authenticatedRequest<GetIndividualLeaderboardResponse>({
            url: `${Endpoints.INDIVIDUAL_LEADERBOARD}?per_page=${pagination.perPage || 25}&page=${pagination.page}`,
            method: "GET",
        });

        return resp.data;
    }
}