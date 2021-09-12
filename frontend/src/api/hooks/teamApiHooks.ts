import {useMutation, UseMutationResult, useQuery, UseQueryResult} from "react-query";
import {
    CreateTeamRequest,
    CreateTeamResponse,
    GetUsersTeamResponse, JoinTeamRequest, LeaveTeamRequest,
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

export const useLeaveTeamMutation = (): UseMutationResult<void, unknown, LeaveTeamRequest> => {
    return useMutation(
        ["LeaveTeamMutation"],
        async (request: LeaveTeamRequest) => TeamApiService.leaveTeam(request),
        {
            onError: () => { toastPopper({ message: "Leaving team failed!" }) },
        }
    );
};

export const useJoinTeamMutation = (): UseMutationResult<void, unknown, JoinTeamRequest> => {
    return useMutation(
        ["JoinTeamMutation"],
        async (request: JoinTeamRequest) => TeamApiService.joinTeam(request),
        {
            onError: () => { toastPopper({ message: "Joining team failed!" }) },
        }
    );
};