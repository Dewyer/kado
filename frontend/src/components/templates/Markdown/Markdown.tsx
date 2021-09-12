import React from "react";
import styles from "./Markdown.module.scss";
import ReactMarkdown from "react-markdown";
import classNames from "classnames";

export interface MarkdownProps {
    className?: string;
    textContents?: string;
}

export const Markdown: React.FC<MarkdownProps> = (props) => {
    const classes = classNames(styles.Markdown, props.className);
    if (props.children) {
        return (
            <div className={classes}>
                <ReactMarkdown>
                    {props.children as unknown as any}
                </ReactMarkdown>
            </div>
        );
    }

    return (
        <div className={classes}>
            <ReactMarkdown
                children={props.textContents || ""}
            />
        </div>
    );
};
