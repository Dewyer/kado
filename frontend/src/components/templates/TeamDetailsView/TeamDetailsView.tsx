import React from "react";
import styles from "./TeamDetailsView.module.scss";
import {TeamFullyPopulatedDto} from "../../../typings/api";
import {useSelector} from "react-redux";
import {makeSelectUser} from "../../../store/selectors/global.selectors";
import {ViewJoinCode} from "./ViewJoinCode";

export interface TeamDetailsViewProps {
    team: TeamFullyPopulatedDto,
}

export const TeamDetailsView: React.FC<TeamDetailsViewProps> = (props) => {
    const {
        team,
    } = props;

    const user = useSelector(makeSelectUser());
    const userIsOwner = team.owner_user?.id === user?.id;

    return (
        <>
            <h2 className="ui header">Team: {team.name}</h2>
            {userIsOwner ? <span>You are the owner of this team.</span> : <span>{team.owner_user?.username} is the owner of the team!</span>}
            {userIsOwner && <ViewJoinCode team={team} />}
        </>
    );
};
