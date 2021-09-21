import React, {useCallback, useRef} from "react";
import { Button } from "semantic-ui-react";
import styles from "./CodeUpload.module.scss";
import {useUploadCodeMutation} from "src/api/hooks/submissionsApiHooks";
import {toastPopper} from "src/helpers/toastPopper";
import {queryClient} from "src/api/queryClient";
import {SubmissionDto} from "src/typings/api";

export interface CodeUploadProps {
    problemCodeName: string,
    submission: SubmissionDto,
}

export const CodeUpload: React.FC<CodeUploadProps> = (props) => {
    const { submission, problemCodeName } = props;

    const inputEl = useRef<HTMLInputElement | null>(null);
    const fileUploadMutation = useUploadCodeMutation();

    const onFileSelection = useCallback(async (files: FileList | null) => {
        const file = files?.item(0);
        if (!file) {
            toastPopper({ message: "Please select a file to upload!" });
            return;
        }

        await fileUploadMutation.mutateAsync({
            file,
            originalName: file.name,
            problemCodeName,
        });
        await queryClient.invalidateQueries("FetchProblemDetails");
    }, [fileUploadMutation, problemCodeName]);

    const alreadyUploaded = !!submission.proof_file_original_name;

    return (
        <div className={styles.CodeUpload}>
            <h3 className={"ui header"}>Upload your code</h3>
            {!alreadyUploaded ? <p>
                You completed this problem so now please upload the last working version of your solution code (no libraries or integration code) <br />
                Choose a .zip file to upload, we won't share this with anyone, any we are only going to use this to keep the competition fair.
            </p> : <p>Thanks for uploading your solution.</p>}

            <Button
                loading={fileUploadMutation.isLoading}
                disabled={fileUploadMutation.isLoading || !!submission.proof_file_original_name}
                content="Upload"
                labelPosition="left"
                icon="file"
                onClick={() => {
                    if (!inputEl.current)
                        return;

                    inputEl.current.click();
                }}
            />
            <input
                ref={inputEl}
                type="file"
                hidden
                onChange={(ev) => {onFileSelection(ev.target.files)}}
            />
            {alreadyUploaded && <p>✔️ Uploaded: {submission.proof_file_original_name}</p>}

        </div>
    );
};
