import {Endpoints} from "src/api/endpoints";
import {ApiService} from "../ApiService";

export type FileType = Blob;

export interface CodeUploadRequest {
    problemCodeName: string,
    originalName: string,
    file: FileType,
}

export abstract class SubmissionsApiService {
    public static async uploadCode(rqs: CodeUploadRequest): Promise<void> {
        const formData = new FormData();
        formData.append("file", rqs.file);

        const resp = await ApiService.authenticatedRequest({
            url: `${Endpoints.UPLOAD_CODE}/${rqs.problemCodeName}/${rqs.originalName}`,
            method: "POST",
            headers: {
                "Content-Type": "multipart/form-data",
            },
            data: formData,
        });

        if (!!(resp.data as any).error) {
            throw new Error("File upload error!");
        }
    }
}