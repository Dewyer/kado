import {useMutation, UseMutationResult} from "react-query";
import {toastPopper} from "../../helpers/toastPopper";
import {SubmissionsApiService} from "../apis/SubmissionsApiService";
import {FileType} from "../apis/SubmissionsApiService";

export const useUploadCodeMutation = (file: FileType): UseMutationResult<unknown, unknown, void> => {
    return useMutation(
        ["UploadCodeMutation"],
        async () => SubmissionsApiService.uploadCode(file),
        {
            onError: () => { toastPopper({ message: "Uploading your code failed! (Are you sure its a .zip file smaller than 3MB?)" }) },
        }
    );
};

