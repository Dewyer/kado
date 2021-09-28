import React from "react";
import {PageLayout} from "src/components/templates/PageLayout/PageLayout";
import {ViewAndRefreshApiToken} from "src/components/molecules/ViewAndRefreshApiToken/ViewAndRefreshApiToken";
import {Markdown} from "../../templates/Markdown/Markdown";
import styles from "./ApiGuidePage.module.scss";

const pageContent = `
### Api Guide

You might be wondering how you can submit a solution to earn points, this is the topic we will cover on this page.

In this competition, one of our goals was to let you use your favorite language, framework, library, environment, etc...
and also to grade solutions on **correctness** and not execution time or memory usage.

With these constraints in mind, we decided not to run your programs in a sandbox environment but to let you run them for us, we are just interested in seeing your programs give the correct answers.

The communication between your program and our verification programs is done through an **HTTP API** (more about them in general [here](https://searchapparchitecture.techtarget.com/definition/RESTful-API)).

#### Concepts
- JSON - javascript object notation (more about it [here](https://www.w3schools.com/js/js_json_intro.asp)) also the format we use to exchange data
- Integration - the part of your program that implements this communication interface with our servers
- Problem - a problem to be solved (listed on the Problems page)
- Submission - an attempt made to give a correct solution to a problem
- Test - part of a solution, that includes us giving you a JSON input, and you giving us back a JSON output. Each submission can contain multiple tests. (Test inputs and correctness can be dependent on outputs given to previous tests too)

#### Submission process
- You write your program that contains both the integration and the solution part.
- Your program creates a new solution and reads how many tests it has to perform to complete it
- Your program requests test inputs
- Your program sends a request containing the desired output to the inputs gotten above
- Repeat as many times as you are required to according to your submission
- If you give a valid output to all tests the submission is considered to be correct
- After you have a correct solution you need to upload (for plagiarism testing), this can either be done on the problem's details page or through the API
- You can attempt to create a solution as many times as the problem allows you to (dependant on difficulty)
- If you want to test your integration or your solution (but don't feel confident enough to do it live) you can create a submission that isn't worth any points and doesn't reduce your remaining solution attempts and just contains sample data.
- At the first incorrect test output your submission is closed
- Submissions and tests can time out if you stop requesting/responding (timeout is ~10s per test but this is subject to change), a timed out test is considered to be incorrect
- You can only have one "in-progress" submission at a time for reach problem
- If your test or the whole submission times out you have to create a new submission 

#### Authentication to the API
For us to know who is trying to create a submission we need you to include your secret **API token** (copiable on the top of the page) in every request you make

in the *X-Api-Token* HTTP header (more about headers [here](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers)).

If you think your API token was compromised (shared it with anyone or showed it to anyone) consider refreshing it and using the new token, your older token will be invalidated.

#### Submission API Specification
The API is available at: *${process.env.REACT_APP_BASE_URL}*

##### Creating a submission - POST - /api/submissions/start-submission
Used to create a new submission.

Request body (Content-Type header must be set to *application/json*):
\`\`\`
{
    "problem": "<problem id>",
    "sample_index": <optional, index of the sample you want to use, don't include this field for live submissions>
}
\`\`\`

Response body
\`\`\`
{
    "submission": {
        "id": "<submision id>",
        "test_count": <how many tests your solution will have to complete to be considered completed>,
        "sample_index": <the sample index if you specified any>,
        "started_at": <the UTC timestamp your solution was started at>,
    }
}
\`\`\`

##### Starting a test (getting the test input) - PUT - /api/submissions/test
Used to get a new test for a submission, has to be called *test_count* times.

Request body (Content-Type header must be set to *application/json*):
\`\`\`
{
    "submission": "<submision id>"
}
\`\`\`

Response body
\`\`\`
{
    "test_id": "<test id>",
    "deadline": <the deadline for test result submission as a UTC timestamp>,
    "input": <program input JSON>
}
\`\`\`

##### Submiting test results (sending test output) - POST - /api/submissions/test/<test_id>
Used to submit test output for a previously started test.

Request body (Content-Type header must be set to *application/json*):
\`\`\`
{
    "output": <output generated for the input JSON>
}
\`\`\`

Response body
\`\`\`
{
    "correct": <true if the test output was correct>
}
\`\`\`

After you complete all tests for a given submission if you go to the problem's details page you should see your submission as correct and you can upload your code.

##### Uploading your code (fairness guarantee) - POST - /files/api/submissions/code-upload/<problem_id>/<original_name>
Used to uplaod your solution code after a correct submission.
- problem_id = the id of the problem you just solved
- original_name = some name for the uploaded file (only you will be able to see this)

Request body (Content-Type must be *multipart/form-data;*): more about this [here](https://programmer.help/blogs/how-does-http-file-upload-work.html)
\`\`\`
file: <your solution's source code in a .zip file>
\`\`\`

Response body (JSON)
\`\`\`
{
    "error": <some error if the file upload failed, an empty string otherwise>
}
\`\`\`

After you uploaded your code the submission process is done and you should see your point total go up :D

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
