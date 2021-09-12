import {useMutation, UseMutationResult, useQuery, UseQueryResult} from "react-query";
import { GetApiTokenResponse } from "../../typings/api";
import {toastPopper} from "src/helpers/toastPopper";
import {ApiTokenApiService} from "../apis/ApiTokenApiService";

export const useFetchApiToken = (): UseQueryResult<GetApiTokenResponse, unknown> => {
    return useQuery(
        ["FetchApiToken"],
        async () => ApiTokenApiService.getApiToken(),
        {
            onError: () => { toastPopper({ message: "Getting api token failed!" }) },
        },
    );
};

export const useRefreshApiTokenMutation = (): UseMutationResult<GetApiTokenResponse, unknown, void> => {
    return useMutation(
        ["RefreshApiTokenMutation"],
        async () => ApiTokenApiService.refreshApiToken(),
        {
            onError: () => { toastPopper({ message: "Refreshing api token failed!" }) },
        }
    );
};