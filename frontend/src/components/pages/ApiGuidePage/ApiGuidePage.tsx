import React from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {ViewAndRefreshApiToken} from "src/components/molecules/ViewAndRefreshApiToken/ViewAndRefreshApiToken";
import {Markdown} from "../../templates/Markdown/Markdown";
import styles from "./ApiGuidePage.module.scss";

const pageContent = `
### Api docs

This will explain the usage of the api, using md!
`;

export const ApiGuidePage: React.FC = () => {
    return (
        <PageLayout>
            This is our api guide, the api is used to send in submissions automatically!
            <ViewAndRefreshApiToken />
            <Markdown
                className={styles.MarkdownContentWrapper}
                textContents={pageContent}
            />
        </PageLayout>
    );
};
