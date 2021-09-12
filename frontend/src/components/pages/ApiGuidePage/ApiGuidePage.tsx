import React from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {ViewAndRefreshApiToken} from "src/components/molecules/ViewAndRefreshApiToken/ViewAndRefreshApiToken";

export const ApiGuidePage: React.FC = () => {
    return (
        <PageLayout>
            This is our api guide to send in submissions automatically!
            <ViewAndRefreshApiToken />
        </PageLayout>
    );
};
