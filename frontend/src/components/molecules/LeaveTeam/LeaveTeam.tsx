import React, {useMemo, useState} from "react";
import styles from "./LeaveTeam.module.scss";
import classNames from "classnames";
import {TeamFullyPopulatedDto, UserDto} from "src/typings/api";
import {Dropdown, DropdownItemProps} from "semantic-ui-react";
import {useSelector} from "react-redux";
import {makeSelectUser} from "../../../store/selectors/global.selectors";

export interface LeaveTeamProps {
    className?: string;
    team: TeamFullyPopulatedDto;
}

const getDropdownOptionForUser = (user: UserDto):  DropdownItemProps => {
    return {
        key: user.id,
        value: user.id,
        text: user.username,
    };
};

export const LeaveTeam: React.FC<LeaveTeamProps> = (props) => {
    const { team } = props;
    const user = useSelector(makeSelectUser());
    const userIsOwner = team.owner_user?.id === user?.id;

    const [inheritor, setInheritor] = useState<string>();

    const inheritorOptions = useMemo(() => {
        return team.members
            .filter((usr) => usr.id !== user?.id)
            .map((user) => getDropdownOptionForUser(user));
    },[team.members]);
    const inheritorSelectionVisible = userIsOwner && !!inheritorOptions.length;
    const leaveTeamDisabled = inheritorSelectionVisible && !inheritor;

    const leaveTeamButtonTitle = inheritorOptions.length === 0 ? "Disband team" : "Leave team";

    return <div className={classNames(styles.LeaveTeam, props.className)}>
        <div><button disabled={leaveTeamDisabled} className="ui red button">{leaveTeamButtonTitle}</button></div>

        {inheritorSelectionVisible && <>
            <label>Select the user to inherit the team:</label>
            <div>
                <Dropdown
                    clearable
                    placeholder={"Team member"}
                    selection
                    value={inheritor}
                    onChange={(_ev, data) => {
                        setInheritor(data.value as unknown as string);
                    }}
                    options={inheritorOptions}
                />
            </div>
        </>}
    </div>;
};