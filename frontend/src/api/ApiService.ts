export abstract class ApiService {

    public static async checkUser(payload: { token: string }): Promise<{ exists: boolean }> {
        return {
            exists: false,
        };
    }
}