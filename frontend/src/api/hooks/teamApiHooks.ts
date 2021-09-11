import {useQuery, UseQueryResult} from "react-query";
import {GetUsersTeamResponse} from "../../typings/api";
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