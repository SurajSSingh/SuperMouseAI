// TODO: Need to have this prevent building when not `dev`
import { dev } from '$app/environment';

export const load = () => {
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