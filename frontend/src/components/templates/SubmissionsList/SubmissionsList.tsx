import React from "react";
import {SubmissionDto} from "src/typings/api";
import styles from "./SubmissionsList.module.scss";

export interface SubmissionsListProps {
    submissions: SubmissionDto[],
}

export interface ListItemProps {
    submission: SubmissionDto,
    index: number,
}

const ListItem: React.FC<ListItemProps> = (props) => {
    const {
        submission,
        index,
    } = props;

    const done = submission.correct !== null;
    const took = Math.floor((submission.finished_at || 0) - submission.started_at);
    let tookStr = submission.finished_at ? `took: ${took}s` : 'Timed out';
    const doneRow = <>{submission.correct ? "✔️" : "❌"} - {tookStr}, {submission.correct ? "Good job 🤓" : ""}</>
    const notDoneRow = <>didn't finish evaluation</>;

    return (
        <div className="item">
            <div className="content">
                <div className="header">{index}. submission</div>
                {done ? doneRow : notDoneRow}
            </div>
        </div>
    );
};


export const SubmissionsList: React.FC<SubmissionsListProps> = (props) => {
    const { submissions } = props;

    return !!submissions.length ? (
        <div className={styles.SubmissionsList}>
            <h3 className={"ui header"}>Submissions: </h3>
            <div className="ui celled list">
                {submissions.map((sub, ii) => <ListItem key={sub.id} submission={sub} index={ii} />)}
            </div>
        </div>
    ) : null;
};
