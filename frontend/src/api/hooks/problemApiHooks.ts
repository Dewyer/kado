import {useQuery, UseQueryResult} from "react-query";
import {GetProblemsResponse} from "../../typings/api";
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