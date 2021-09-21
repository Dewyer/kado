import {useQuery, UseQueryResult} from "react-query";
import {GetProblemDetailsResponse, GetProblemsResponse} from "../../typings/api";
import {toastPopper} from "src/helpers/toastPopper";
import {ProblemApiService} from "src/api/apis/ProblemApiService";

export const useFetchProblems = (): UseQueryResult<GetProblemsResponse, unknown> => {
    return useQuery(
        ["FetchProblems"],
        async () => ProblemApiService.getProblems(),
        {
            onError: () => { toastPopper({ message: "Getting available problems failed!" }) },
        },
    );
};

export const useFetchProblemDetails = (codeName: string): UseQueryResult<GetProblemDetailsResponse, unknown> => {
    return useQuery(
        ["FetchProblemDetails", codeName],
        async () => ProblemApiService.getProblemDetails(codeName),
        {
            enabled: !!codeName,
            keepPreviousData: true,
            onError: () => { toastPopper({ message: "Getting problem details failed!" }) },
        },
    );
};