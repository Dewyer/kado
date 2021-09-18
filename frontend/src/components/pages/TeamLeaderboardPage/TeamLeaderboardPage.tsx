import React, {useState} from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import styles from "./TeamLeaderboardPage.module.scss";
import {useFetchTeamLeaderboard} from "src/api/hooks/leaderboardApiHooks";
import {ColumnDefinition, PaginatedTable} from "src/components/templates/PaginatedTable/PaginatedTable";
import {TeamLeaderboardEntryDto} from "src/typings/api";

const getLeaderboardColumns = (): ColumnDefinition<TeamLeaderboardEntryDto>[]  => {
	return [
		{
			name: "Rank",
			cell: (team) => <>{team.rank}.</>
		},
		{
			name: "Points",
			cell: (team) => <>{team.points} pts</>
		},
		{
			name: "Team",
			cell: (team) => <>{team.name}</>
		},
	];
};

export const TeamLeaderboardPage: React.FC = () => {
	const [page, setPage] = useState(0);
	const leaderboardQuery = useFetchTeamLeaderboard({
		perPage: 25,
		page,
	});

	const userTeamRank = leaderboardQuery.data?.user_team_ranking;

	return (
		<PageLayout contentClassName={styles.TeamLeaderboardPage}>
			<h3 className={"ui header"}>Team leaderboard</h3>
			{userTeamRank && <span>Your team: {userTeamRank.rank}. - {userTeamRank.name} - {userTeamRank.points}pts</span>}

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
