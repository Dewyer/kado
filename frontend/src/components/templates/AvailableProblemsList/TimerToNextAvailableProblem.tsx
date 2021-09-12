import React, {useState} from "react";
import Timer from "react-compound-timer";

export interface TimerToNextAvailableProblemProps {
    nextProblemAt: string;
}

export const TimerToNextAvailableProblem: React.FC<TimerToNextAvailableProblemProps> = (props) => {
    const { nextProblemAt } = props;
    const [now] = useState(new Date().getTime());
    const nextAt = new Date(nextProblemAt).getTime();
    const timeTill = nextAt - now;

    const timeTillMoreThen = [
        timeTill >= 24*60*60*1000,
        timeTill >= 60*60*1000,
        timeTill >= 60*1000,
    ];
    return (
        <h4 className={"ui header"}>{"Next problem available in: "}
            <Timer
                initialTime={timeTill}
                direction="backward"
            >
                {() => (
                    <React.Fragment>
                        {timeTillMoreThen[0] && (<><Timer.Days /> {"days "}</>)}
                        {timeTillMoreThen[1] && (<><Timer.Hours /> {"hours "}</>)}
                        {timeTillMoreThen[2] && (<><Timer.Minutes /> {"minutes "}</>)}
                        <Timer.Seconds /> {"seconds"}
                    </React.Fragment>
                )}
            </Timer>
            {"."}
        </h4>
    );
};