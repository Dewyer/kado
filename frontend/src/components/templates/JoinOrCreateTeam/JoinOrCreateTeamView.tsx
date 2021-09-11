import React from "react";
import styles from "./JoinOrCreateTeamView.module.scss";
import {JoinTeam} from "../../molecules/JoinTeam/JoinTeam";
import {CreateTeam} from "../../molecules/CreateTeam/CreateTeam";

export interface JoinOrCreateTeamViewProps {
}

export const JoinOrCreateTeamView: React.FC<JoinOrCreateTeamViewProps> = () => {
    return (
        <>
            <span className={styles.JoinOrCreateTeamViewTitle}>You don't seem to have a team yet. Join one using the team join code or create a new team below.</span>
            <JoinTeam />
            <div className="ui divider" />
            <CreateTeam />
        </>
    );
};
