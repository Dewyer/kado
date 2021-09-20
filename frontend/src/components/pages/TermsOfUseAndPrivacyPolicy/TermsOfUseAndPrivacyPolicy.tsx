import React from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {Markdown} from "../../templates/Markdown/Markdown";

const pageContent = `
### Terms of use

I can have your kidneys.

### Privacy policy

I can have your kidneys.
`;


export const TermsOfUseAndPrivacyPolicy: React.FC = () => {
	return (
		<PageLayout>
			<Markdown
				textContents={pageContent}
			/>
		</PageLayout>
	);
};
