import {useMutation, UseMutationResult} from "react-query";
import {toastPopper} from "../../helpers/toastPopper";
import {CodeUploadRequest, SubmissionsApiService} from "../apis/SubmissionsApiService";

export const useUploadCodeMutation = (): UseMutationResult<unknown, unknown, CodeUploadRequest> => {
    return useMutation(
        ["UploadCodeMutation"],
        async (rqs: CodeUploadRequest) => SubmissionsApiService.uploadCode(rqs),
        {
            onError: () => { toastPopper({ message: "Uploading your code failed! (Are you sure its a .zip file smaller than 3MB?)" }) },
        }
    );
};

