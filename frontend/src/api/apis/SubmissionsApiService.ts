import {Endpoints} from "src/api/endpoints";
import {ApiService} from "../ApiService";

export type FileType = Blob;

export abstract class SubmissionsApiService {
    public static async uploadCode(file: FileType): Promise<void> {
        const formData = new FormData();
        formData.append("file", file);

        const resp = await ApiService.authenticatedRequest({
            url: Endpoints.UPLOAD_CODE,
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