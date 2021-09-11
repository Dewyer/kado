import {useMutation, UseMutationResult, useQuery, UseQueryResult} from "react-query";
import {
    CreateTeamRequest,
    CreateTeamResponse,
    GetUsersTeamResponse,
} from "src/typings/api";
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

export const useCreateTeamMutation = (): UseMutationResult<CreateTeamResponse, unknown, CreateTeamRequest> => {
    return useMutation(
        ["CreateTeamMutation"],
        async (request: CreateTeamRequest) => TeamApiService.createTeam(request),
        {
            onError: () => { toastPopper({ message: "Creating team failed!" }) },
        }
    );
};