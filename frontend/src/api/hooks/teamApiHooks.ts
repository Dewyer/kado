import {useMutation, UseMutationResult, useQuery, UseQueryResult} from "react-query";
import {AuthorizingResponse, GetUsersTeamResponse, RegisterRequest} from "../../typings/api";
import {AuthApiService} from "../apis/AuthApiService";
import {toastPopper} from "../../helpers/toastPopper";
import {TeamApiService} from "../apis/TeamApiService";

export const useFetchUserTeam = (): UseQueryResult<GetUsersTeamResponse, unknown> => {
    return useQuery(
        ["FetchUserTeam"],
        async () => TeamApiService.getTeam(),
        {
            onError: () => { toastPopper({ message: "Getting user's team failed!" }) },
        },
    );
};