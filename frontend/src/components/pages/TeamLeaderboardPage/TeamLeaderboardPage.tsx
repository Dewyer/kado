import React, {useState} from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import styles from "./TeamLeaderboardPage.module.scss";

export const TeamLeaderboardPage: React.FC = () => {
	// const [page, setPage] = useState(0);

	return (
		<PageLayout contentClassName={styles.TeamLeaderboardPage}>
			<h3 className={"ui header"}>Team leaderboard</h3>
			<code>// TODO</code>
		</PageLayout>
	);
};
