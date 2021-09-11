import React from "react";
import styles from "./TeamPage.module.scss";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {useFetchUserTeam} from "../../../api/hooks/teamApiHooks";

const PageLoader = () => (
    <div className="ui active dimmer">
        <div className="ui text loader">Loading user's team ...</div>
    </div>
);

export const TeamPage: React.FC = () => {
    const fetchTeamResult = useFetchUserTeam();

    if (fetchTeamResult.isLoading) {
        return (
            <PageLayout>
                <PageLoader />
            </PageLayout>
        );
    }

    return (
        <PageLayout>
            {fetchTeamResult.data?.team?.name} team wow!
        </PageLayout>
    );
};
