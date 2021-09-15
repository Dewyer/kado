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
			name: "Username",
			cell: (user) => <>{user.username}</>
		},
		{
			name: "Team",
			cell: (user) => <>{user.team_name}</>
		}
	];
};

export const IndividualLeaderboardPage: React.FC = () => {
	const [page, setPage] = useState(0);
	const leaderboardQuery = useFetchIndividualLeaderboard({
		perPage: 25,
		page,
	});

	return (
		<PageLayout contentClassName={styles.IndividualLeaderboardPage}>
			<h3 className={"ui header"}>Individual leaderboard</h3>

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
