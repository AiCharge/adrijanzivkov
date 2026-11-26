import { createInertiaApp } from '@inertiajs/vue3';
import createServer from '@inertiajs/vue3/server';
import { resolvePageComponent } from 'laravel-vite-plugin/inertia-helpers';
import { createSSRApp, DefineComponent, h } from 'vue';
import { renderToString } from 'vue/server-renderer';
import type { Config } from 'ziggy-js';
import { ZiggyVue } from 'ziggy-js';
import { reveal } from './lib/reveal';

const appName = import.meta.env.VITE_APP_NAME || 'Laravel';

createServer(
    (page) =>
        createInertiaApp({
            page,
            render: renderToString,
            title: (title) => (title ? `${title} - ${appName}` : appName),
            resolve: resolvePage,
            setup: ({ App, props, plugin }) => {
                const ziggy = page.props.ziggy as Config & { location: string };

                return createSSRApp({ render: () => h(App, props) })
                    .use(plugin)
                    .directive('reveal', reveal)
                    .use(ZiggyVue, {
                        ...ziggy,
                        location: new URL(ziggy.location),
                    });
            },
        }),
    { cluster: true },
);

function resolvePage(name: string) {
    const pages = import.meta.glob<DefineComponent>('./pages/**/*.vue');

    return resolvePageComponent<DefineComponent>(`./pages/${name}.vue`, pages);
}
