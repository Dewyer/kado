import {toast} from "react-toastify";
import {CustomToast} from "../components/molecules/CustomToast/CustomToast";

export interface ToastPopperProps {
    message: string;
}

export const toastPopper = (props: ToastPopperProps) => {
    toast(<CustomToast
        message={props.message}
    />, {
        position: "bottom-left",
    });
};