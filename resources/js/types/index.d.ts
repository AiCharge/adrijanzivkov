import type { AuthUserInterface } from '@/interfaces/auth-user-interface';
import type { Config } from 'ziggy-js';

export type AppPageProps<T extends Record<string, unknown> = Record<string, unknown>> = T & {
    name: string;
    auth: {
        user: AuthUserInterface | null;
    };
    ziggy: Config & { location: string };
};
