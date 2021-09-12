import React from "react";
import styles from "./TeamPage.module.scss";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {useFetchUserTeam} from "../../../api/hooks/teamApiHooks";
import {TeamDetailsView} from "../../templates/TeamDetailsView/TeamDetailsView";
import {JoinOrCreateTeamView} from "../../templates/JoinOrCreateTeam/JoinOrCreateTeamView";
import {PageLoader} from "../../templates/PageLoader/PageLoader";

export const TeamPage: React.FC = () => {
    const fetchTeamResult = useFetchUserTeam();

    if (fetchTeamResult.isLoading) {
        return (
            <PageLayout>
                <PageLoader message="Loading user's team ..." />
            </PageLayout>
        );
    }

    return (
        <PageLayout contentClassName={styles.TeamPageContainer}>
            {fetchTeamResult.data?.team ?
                <TeamDetailsView team={fetchTeamResult.data.team} /> :
                <JoinOrCreateTeamView />
            }
        </PageLayout>
    );
};
