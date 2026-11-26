<script setup lang="ts">
import { index as newsIndex } from '@/actions/App/Http/Controllers/NewsController';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import type { NewsInterface } from '@/interfaces/news-interface';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import dayjs from '@/lib/dayjs';
import { Head, Link } from '@inertiajs/vue3';

interface Props {
    news: NewsInterface;
}

defineProps<Props>();

defineOptions({
    layout: DefaultLayout,
});
</script>

<template>
    <Head :title="news.title" />

    <article class="py-10 md:py-16">
        <AppContainer>
            <div class="mx-auto max-w-3xl">
                <Link
                    :href="newsIndex.url()"
                    class="group inline-flex animate-rise items-center gap-2 text-sm font-semibold text-gray-600 transition-colors hover:text-primary"
                >
                    <svg
                        class="h-4 w-4 transition-transform duration-300 group-hover:-translate-x-1"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 12H5m6 6-6-6 6-6" />
                    </svg>
                    Назад кон новости
                </Link>

                <header class="mt-10 flex flex-col gap-5">
                    <time
                        :datetime="news.created_at"
                        class="animate-rise text-xs font-semibold tracking-[0.2em] text-primary uppercase [animation-delay:80ms]"
                    >
                        {{ dayjs(news.created_at).format('D MMMM YYYY') }}
                    </time>
                    <h1
                        class="animate-rise text-3xl leading-[1.1] font-bold tracking-tight text-balance text-gray-900 [animation-delay:160ms] md:text-5xl"
                    >
                        {{ news.title }}
                    </h1>
                </header>
            </div>

            <div class="mx-auto mt-10 max-w-5xl animate-rise overflow-hidden rounded-2xl bg-gray-100 [animation-delay:240ms] md:mt-12">
                <img :src="news.main_image_url ?? undefined" :alt="news.title" class="aspect-video w-full animate-zoom-out object-cover" />
            </div>

            <div v-reveal class="prose-content mx-auto max-w-3xl pt-12 md:pt-16" v-html="news.content"></div>
        </AppContainer>
    </article>
</template>
