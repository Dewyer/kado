import React from "react";
import styles from "./TeamFinder.module.scss";
import {TeamFullyPopulatedDto} from "../../../typings/api";

export interface TeamFinderProps {
    team: TeamFullyPopulatedDto,
}

export const TeamFinder: React.FC<TeamFinderProps> = () => {
    return (
        <span>Team finder!</span>
    );
};
