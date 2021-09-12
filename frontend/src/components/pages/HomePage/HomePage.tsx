import React from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {Markdown} from "../../templates/Markdown/Markdown";

const pageContent = `
### Snap Problems

This is a wonderful homepage that uses markdown!
`;


export const HomePage: React.FC = () => {
	return (
		<PageLayout>
			<Markdown
				textContents={pageContent}
			/>
		</PageLayout>
	);
};
