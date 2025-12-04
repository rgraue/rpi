export interface Config {
    SERVER_URL: string;
}

// if there is an config.active.json. use that instead.
// meant for environment specific settings
export const config = async (): Promise<Config> => {
    try {
        return await import('../config.active.json'!);
    } catch {
        return await import('../config.json');
    }
}