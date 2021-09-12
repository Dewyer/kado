import React from "react";
import { Link } from "react-router-dom";
import {GetProblemsResponse, ProblemDto} from "src/typings/api";
import {GLOBAL_ROUTES} from "src/routing/routingConstants";
import styles from "./AvailableProblemsList.module.scss";

export interface ProblemListItemProps {
    problem: ProblemDto;
}

const ProblemListItem: React.FC<ProblemListItemProps> = (props) => {
    const { problem } = props;

    return (
        <Link
            to={`${GLOBAL_ROUTES.PROBLEM_DETAILS}/${problem.code_name}`}
            className="item"
        >
            {problem.name}
            <span className={styles.LinkListItemText}>
                ({"💡".repeat(problem.difficulty)}
                {"   "}
                {problem.base_point_value} pts)
            </span>
        </Link>
    );
};

export interface AvailableProblemsListProps {
    problemsResponse: GetProblemsResponse,
}

export const AvailableProblemsList: React.FC<AvailableProblemsListProps> = (props) => {
    return (
        <div className={styles.AvailableProblemsList}>
            <h3 className={"ui header"}>Problems: </h3>
            <div className="ui ordered list">
                {props.problemsResponse.problems.map((pr) => <ProblemListItem problem={pr} />)}
            </div>
        </div>
    );
};