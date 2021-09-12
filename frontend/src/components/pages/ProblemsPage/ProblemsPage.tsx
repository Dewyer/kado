import React from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {PageLoader} from "src/components/templates/PageLoader/PageLoader";
import {useFetchProblems} from "src/api/hooks/problemApiHooks";
import {AvailableProblemsList} from "src/components/templates/AvailableProblemsList/AvailableProblemsList";

export const ProblemsPage: React.FC = () => {
    const problemsQuery = useFetchProblems();

    if (problemsQuery.isLoading || !problemsQuery.data) {
        return (
            <PageLayout>
                <PageLoader message="Loading problems ..." />
            </PageLayout>
        );
    }

    return (
        <PageLayout>
            <AvailableProblemsList
                problemsResponse={problemsQuery.data}
            />
        </PageLayout>
    );
};
