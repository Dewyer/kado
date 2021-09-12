import {
    GetProblemsResponse,
} from "src/typings/api";
import {Endpoints} from "src/api/endpoints";
import {ApiService} from "../ApiService";

export abstract class ProblemApiService {
    public static async getProblems(): Promise<GetProblemsResponse> {
        const resp = await ApiService.authenticatedRequest<GetProblemsResponse>({
            url: Endpoints.GET_PROBLEMS,
            method: "GET",
        });

        return resp.data;
    }
}