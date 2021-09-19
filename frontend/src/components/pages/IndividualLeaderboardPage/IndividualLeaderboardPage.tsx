import React, {useState} from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {useFetchIndividualLeaderboard} from "src/api/hooks/leaderboardApiHooks";
import {ColumnDefinition, PaginatedTable} from "src/components/templates/PaginatedTable/PaginatedTable";
import {UserLeaderboardEntryDto} from "src/typings/api";
import styles from "./IndividualLeaderboardPage.module.scss";

const getLeaderboardColumns = (): ColumnDefinition<UserLeaderboardEntryDto>[]  => {
	return [
		{
			name: "Rank",
			cell: (user) => <>{user.rank}.</>
		},
		{
			name: "Points",
			cell: (user) => <>{user.individual_points} pts</>
		},
		{
			name: "Username / Team",
			cell: (user) => <>{user.username === 'anonymous' ? <i>{user.username}</i> : user.username} {user.team_name ? ` (${user.team_name})` : ''}</>
		},
	];
};

export const IndividualLeaderboardPage: React.FC = () => {
	const [page, setPage] = useState(0);
	const leaderboardQuery = useFetchIndividualLeaderboard({
		perPage: 25,
		page,
	});

	const userRanking = leaderboardQuery.data?.user_ranking;

	return (
		<PageLayout contentClassName={styles.IndividualLeaderboardPage}>
			<h3 className={"ui header"}>Individual leaderboard</h3>

			{userRanking && <span>You: {userRanking.rank}. - {userRanking.individual_points}pts - {userRanking.username}{userRanking.team_name ? ` - ${userRanking.team_name}` : ''}</span>}

			<PaginatedTable
				page={page}
				rows={leaderboardQuery.data?.leaderboard || []}
				columns={getLeaderboardColumns()}
				pageCount={leaderboardQuery.data?.page_count || 0}
				onSetPage={(newPage) => setPage(newPage)}
			/>
		</PageLayout>
	);
};
