import React from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {Markdown} from "src/components/templates/Markdown/Markdown";

const pageContent = `
### Report a problem

If encounter any problems please reach out to us at: **qpa-challenge@snapsoft.hu**.

We will try to get back to you as soon as possible.
`;


export const ReportProblem: React.FC = () => {
    return (
        <PageLayout>
            <Markdown
                textContents={pageContent}
            />
        </PageLayout>
    );
};
