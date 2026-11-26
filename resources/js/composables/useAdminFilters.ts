import { router } from '@inertiajs/vue3';
import { useDebounceFn } from '@vueuse/core';
import { reactive, watch } from 'vue';

/**
 * Keeps an admin index page's filters in the query string, reloading the page when they change.
 */
export function useAdminFilters<T extends Record<string, string>>(url: string, initialFilters: T) {
    const filters = reactive({ ...initialFilters }) as T;

    const reload = useDebounceFn(() => {
        const query = Object.fromEntries(Object.entries(filters).filter(([, value]) => value !== ''));

        router.get(url, query, { preserveState: true, preserveScroll: true, replace: true });
    }, 300);

    watch(filters, reload);

    return filters;
}
