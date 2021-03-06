import React from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {Markdown} from "../../templates/Markdown/Markdown";

const pageContent = `
### Snap Challenge

This program is part of Schönherz Qpa 2021, and is a competitive programming style, multi-day competition.
You can earn points that get converted to QPA points at the end.

On most days from **2021.09.27 (starting at 00:00) - 2021.10.01** we will release a new problem for you to solve increasing in difficulty.
You can solve these problems with your favorite programming language (more about submissions in the API guide).
You can work alone or in a team but **QPA points are only awarded to real QPA teams** (We expect teams to correspond to a real Schönherz QPA team).

The individual (contributing to a team or not) with the highest rank at the end will also be eligible for a special prize at the Snapsoft booth on 2021.10.05.

### Rules
- Requirements (breaking these rules will result in **a ban**)
	- You must have an unoffensive username that is your own (not knowingly used by someone else)
	- You must create your unique problem solution, this doesn't include:
		- the integration with the solutions API (more on this in the API guide)
		- any other tools you might use that could be considered general-purpose ex: libraries, CLI tools, etc... (not made to solve our exact problems), questionable cases will be subject to individual review
	- You upload misleading code on purpose after correctly completing a submission (more about this in the API Guide)
- Structure
	- Each problem is worth a number of points depending on its difficulty (xx pts on the website)
	- Individuals are expected to register and then join a team (to earn QPA points)
	- Each individual has their own points separate from the team's points they are in
	- Points gained before joining a team will not count toward a team's overall points
	- Teams can gain points for each individual's correct solution to a problem but the following additional rules apply:
		- Subsequent solutions to the same problem give fewer and fewer points, so the amount gained for a new solution is calculated according to the following formula: \`P * e^(-0.2 * (C+1))\` where P is the number of points assigned to the problem and C is how many correct previous solutions were submitted.
		- Each individual in a team is expected to come up with a solution on its own, without any help from other teammates, submitting the same solution is considered to be plagiarism, we will be monitoring this very seriously.
	- Rankings for teams/individuals with the same amount of points are decided by looking at the time of the latest correct submission (earliest wins)

Have fun and happy problem-solving, if encounter any problems please reach out to us at: *qpa-challenge@snapsoft.hu*.

We also recommend using a desktop computer to use the site for the best experience.

### Updates
- 09.27-16:10 - We solved a file uploading issue, now you should be able to upload your solutions, our apologies for the inconvenience.
- 09.28-10:00 - We fixed a problem that caused submissions to never time out correctly, now if you see the 'didn't finish evaluation' status on your submission, wait a bit for it to time out, and then you should be able to create a new submission.
- 09.30-09:09 - We fixed the \`King Pinned\` problem's incorrect sample, our apologies.

### 10.01 - Next task

Hi players, as you probably noticed we didn't release a problem yesterday, we are sorry that our original plans couldn't come to fruition.
Today (10.01) we are going to release the **last task** in the competition.

Sorry for all the hiccups.
Thank you all for participating 😀 Hope you enjoyed it (maybe see you next year 😉 ?) 
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
