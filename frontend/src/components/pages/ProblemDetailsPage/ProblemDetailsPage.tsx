import React from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {Redirect, useParams} from "react-router-dom";
import styles from "./ProblemDetailsPage.module.scss"
import {useFetchProblemDetails} from "../../../api/hooks/problemApiHooks";
import {PageLoader} from "../../templates/PageLoader/PageLoader";
import {GLOBAL_ROUTES} from "../../../routing/routingConstants";

export const ProblemDetailsPage: React.FC = () => {
    const { codeName } = useParams<{ codeName: string }>();
    const problemQuery = useFetchProblemDetails(codeName || "");

    if (!codeName) {
        return <Redirect to={GLOBAL_ROUTES.HOME} />
    }

    if (problemQuery.isLoading || !problemQuery.data) {
        return (
            <PageLayout>
                <PageLoader message="Loading problem details ..." />
            </PageLayout>
        );
    }
    const problem = problemQuery.data.problem;

    return (
        <PageLayout contentClassName={styles.ProblemDetailsPage}>
            <h2 className={"ui header"}>{problem.name}</h2>
            <p>{
                problem.problem_statement.problem_statement
            }</p>
        </PageLayout>
    );
};
