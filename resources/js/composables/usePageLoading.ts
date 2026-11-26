import { router } from '@inertiajs/vue3';
import { computed, ref } from 'vue';

/**
 * Where the page visit in flight is going. A reload of this page (a tab, filter, search, sort or page of results)
 * is set at once, so its tables never show the old rows or an empty state while the new rows load; a move to
 * another page only once it takes longer than a blink, so quick answers don't flash a loader. Form submits, async
 * reloads and prefetches leave the page as it is, so they don't count.
 */
const pendingUrl = ref<URL | null>(null);
let showAfterDelay: ReturnType<typeof setTimeout> | undefined;

const loadsPage = (visit: { method: string; async: boolean; prefetch: boolean }): boolean =>
    visit.method === 'get' && !visit.async && !visit.prefetch;

router.on('start', ({ detail: { visit } }) => {
    if (!loadsPage(visit)) {
        return;
    }

    clearTimeout(showAfterDelay);

    if (visit.url.pathname === window.location.pathname) {
        pendingUrl.value = visit.url;

        return;
    }

    showAfterDelay = setTimeout(() => (pendingUrl.value = visit.url), 200);
});

router.on('finish', ({ detail: { visit } }) => {
    if (!loadsPage(visit)) {
        return;
    }

    clearTimeout(showAfterDelay);
    pendingUrl.value = null;
});

/**
 * `isReloading` is this page fetching new rows (a tab, filter, search, sort or page of results);
 * `isLeaving` is a move to another page.
 */
export const usePageLoading = () => ({
    isReloading: computed(() => pendingUrl.value?.pathname === window.location.pathname),
    isLeaving: computed(() => pendingUrl.value !== null && pendingUrl.value.pathname !== window.location.pathname),
});
