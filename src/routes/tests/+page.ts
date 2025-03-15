// TODO(eventually): Need to have this prevent building when not `dev`
import { dev } from '$app/environment';

export type PageLoadType = {
    status: number;
    error: string;
    tests?: undefined;
} | {
    tests: string[];
    status?: undefined;
    error?: undefined;
};

export const load = (): PageLoadType => {
    if (!dev) {
        return {
            status: 404,
            error: 'Not Found'
        };
    }

    return {
        tests: ['notifier']
    };
};