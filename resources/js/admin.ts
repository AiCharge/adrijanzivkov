import '../css/admin.css';

import AdminLayout from '@/layouts/AdminLayout.vue';
import { createInertiaApp } from '@inertiajs/vue3';
import { resolvePageComponent } from 'laravel-vite-plugin/inertia-helpers';
import type { DefineComponent } from 'vue';
import { createApp, h } from 'vue';

const appName = import.meta.env.VITE_APP_NAME || 'Laravel';

/**
 * The admin panel's own app: its own stylesheet and only the admin pages, so the public site's look never mixes with it.
 */
createInertiaApp({
    title: (title) => (title ? `${title} - ${appName}` : appName),
    resolve: (name) => resolvePageComponent(`./pages/${name}.vue`, import.meta.glob<DefineComponent>('./pages/Admin/**/*.vue')),
    layout: (name) => (name.startsWith('Admin/Auth/') ? null : AdminLayout),
    setup({ el, App, props, plugin }) {
        createApp({ render: () => h(App, props) })
            .use(plugin)
            .mount(el);
    },
    progress: {
        color: 'var(--primary)',
    },
});
