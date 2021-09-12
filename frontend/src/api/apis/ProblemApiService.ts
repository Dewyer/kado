import {
    GetProblemDetailsResponse,
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

    public static async getProblemDetails(codeName: string): Promise<GetProblemDetailsResponse> {
        const resp = await ApiService.authenticatedRequest<GetProblemDetailsResponse>({
            url: `${Endpoints.GET_PROBLEM_DETAILS}/${codeName}`,
            method: "GET",
        });

        return resp.data;
    }
}