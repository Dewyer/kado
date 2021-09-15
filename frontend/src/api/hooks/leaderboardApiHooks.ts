import {useQuery, UseQueryResult} from "react-query";
import {GetIndividualLeaderboardResponse} from "../../typings/api";
import {toastPopper} from "../../helpers/toastPopper";
import {LeaderboardApiService} from "../apis/LeaderboardApiService";
import {PaginationOptions} from "../../typings/commonTypes";

export const useFetchIndividualLeaderboard = (pagination: PaginationOptions): UseQueryResult<GetIndividualLeaderboardResponse, unknown> => {
    return useQuery(
        ["FetchIndividualLeaderboard", pagination],
        async () => LeaderboardApiService.getIndividualLeaderboard(pagination),
        {
            onError: () => { toastPopper({ message: "Getting leaderboard failed!" }) },
        },
    );
};