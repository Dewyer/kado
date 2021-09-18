import React from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {ViewAndRefreshApiToken} from "src/components/molecules/ViewAndRefreshApiToken/ViewAndRefreshApiToken";
import {Markdown} from "../../templates/Markdown/Markdown";
import styles from "./ApiGuidePage.module.scss";

const pageContent = `
### Api Guide

You might be wondering how you can submit a solution to earn points, this is the topic we will cover on this page.

In this competition one of our goals was to let you use your favourite language, framework, library, environment etc...
and also to grade solutions on **correctness** and not execution time or memory usage.

With these constraints in mind we decided not to run your programs in a sandbox environment but to let you run it for us, we are just interested in seeing it give the correct answers.

The communication between your program and our verification programs is done through an **HTTP API** (more about them in general [here](https://searchapparchitecture.techtarget.com/definition/RESTful-API)).

#### Concepts
- JSON - javascript object notation (more about it [here](https://www.w3schools.com/js/js_json_intro.asp)) also the format we use to exchange data
- Integration - the part of your program that implements this communication interface with our servers
- Problem - a problem to be solved (listed in the Problems page)
- Submission - an attempt made to give a correct solution to a problem
- Test - part of a solution, that includes us giving you an JSON input, and you giving us back a JSON output. Each submission can contain multiple tests.

#### Submission process
- You write your program that contains both the integration and the solution part.
- Your program creates a new solution and reads how many tests it has to perform to complete it
- Your program requests test inputs
- Your program sends a request containing the desired output to the inputs gotten above
- Repeat as many times as you are required to according to your submission
- If give a valid output to all tests the submission is considered to be correct and you get the points for the problem
- You can attempt to create a solution as many times as the problem allows you to (dependant on difficulty)
- If you want to test your integration or your solution (but don't feel confident enough to do it live) you can create a submission that isn't worth any points and doesn't reduce your remaining solution attempts and just contains sample data.


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
