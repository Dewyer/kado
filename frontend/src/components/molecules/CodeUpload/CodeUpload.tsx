import React, {useRef} from "react";
import { Button } from "semantic-ui-react";
import styles from "./CodeUpload.module.scss";

export const CodeUpload: React.FC = () => {
    const inputEl = useRef<HTMLInputElement | null>(null);

    return (
        <div className={styles.CodeUpload}>
            <h3 className={"ui header"}>Upload your code</h3>
            <p>
                You completed this problem so now please upload the last working version of your solution code (no libraries or integration code) <br />
                Choose a .zip file to upload, we won't share this with anyone, any we are only going to use this to keep the competition fair.
            </p>

            <Button
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
                onChange={(ev) => {console.log(ev.target.files)}}
            />
        </div>
    );
};
