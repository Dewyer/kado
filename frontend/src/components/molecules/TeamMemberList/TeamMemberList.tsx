import React from "react";
import {TeamFullyPopulatedDto, UserDto} from "src/typings/api";
import styles from "./TeamMemberList.module.scss";
import classNames from "classnames";

export interface TeamMemberListProps {
    team: TeamFullyPopulatedDto,
};

export interface ListItemProps {
    user: UserDto,
    isOwner: boolean,
}

const ListItem: React.FC<ListItemProps> = (props) => {
    const {
        user,
        isOwner,
    } = props;

    return (
        <div className="item">
            <div className="content">
                <div className="header">{isOwner ? "👑 " : ""}{user.username}</div>
                On leaderboards: {user.participate_in_leaderboards ? "yes" : "no"}
            </div>
        </div>
    );
};

export const TeamMemberList: React.FC<TeamMemberListProps> = (props) => {
    const {
        team,
    } = props;

    return (
        <div className={styles.TeamMemberList}>
            <h3 className={classNames("ui header", styles.TeamMemberListTitle)}>Members:</h3>
            <div className="ui celled list">
                {team.members.map((member) => <ListItem isOwner={member.id === team.owner_user?.id} key={member.id} user={member} />)}
            </div>
        </div>
    );
};