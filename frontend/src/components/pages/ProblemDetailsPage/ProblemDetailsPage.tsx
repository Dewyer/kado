import React from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {Link, Redirect, useParams} from "react-router-dom";
import styles from "./ProblemDetailsPage.module.scss"
import {useFetchProblemDetails} from "../../../api/hooks/problemApiHooks";
import {PageLoader} from "../../templates/PageLoader/PageLoader";
import {GLOBAL_ROUTES} from "src/routing/routingConstants";
import {Markdown} from "src/components/templates/Markdown/Markdown";

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
            <h2 className={"ui header"}>{problem.name}
                {" "}
                ({"💡".repeat(problem.difficulty)}
                {"   "}
                {problem.base_point_value} pts)
            </h2>
            <span>ID: {problem.code_name}</span>
            <h3 className={"ui header"}>Problem statement:</h3>
            <Markdown
                textContents={problem.problem_statement.problem_statement}
            />
            <span className={styles.ToSolveDisclaimer}>
                To solve this problem follow the instructions found in the <Link to={GLOBAL_ROUTES.API_GUIDE}>Api Guide</Link>.
            </span>
        </PageLayout>
    );
};
