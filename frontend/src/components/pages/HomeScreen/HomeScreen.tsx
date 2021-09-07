import React from "react";
import styles from "./HomeScreen.module.scss";
import {PageLayout} from "../../templates/PageLayout/PageLayout";

export const HomeScreen: React.FC = () => {
	return (
		<PageLayout>
			<div className={styles.homePage}>
				<button className="ui primary button">
					Save
				</button>
			</div>
		</PageLayout>
	);
};
