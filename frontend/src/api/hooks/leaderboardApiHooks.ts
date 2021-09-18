import {useQuery, UseQueryResult} from "react-query";
import {GetIndividualLeaderboardResponse, GetTeamLeaderboardResponse} from "../../typings/api";
import {toastPopper} from "../../helpers/toastPopper";
import {LeaderboardApiService} from "../apis/LeaderboardApiService";
import {PaginationOptions} from "../../typings/commonTypes";

export const useFetchIndividualLeaderboard = (pagination: PaginationOptions): UseQueryResult<GetIndividualLeaderboardResponse, unknown> => {
    return useQuery(
        ["FetchIndividualLeaderboard", pagination],
        async () => LeaderboardApiService.getIndividualLeaderboard(pagination),
        {
            onError: () => { toastPopper({ message: "Getting leaderboard failed!" }) },
            keepPreviousData: true,
        },
    );
};

export const useFetchTeamLeaderboard = (pagination: PaginationOptions): UseQueryResult<GetTeamLeaderboardResponse, unknown> => {
    return useQuery(
        ["FetchTeamLeaderboard", pagination],
        async () => LeaderboardApiService.getTeamLeaderboard(pagination),
        {
            onError: () => { toastPopper({ message: "Getting leaderboard failed!" }) },
            keepPreviousData: true,
        },
    );
};