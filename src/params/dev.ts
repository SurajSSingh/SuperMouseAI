// TODO(eventually): Need to have this prevent building when not `dev`

import { dev } from '$app/environment';
import type { ParamMatcher } from '@sveltejs/kit';

const nameOfRoute = "tests";

export const match: ParamMatcher = (param: string): boolean => {
    return dev && param.includes(nameOfRoute);
};