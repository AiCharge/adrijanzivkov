<script setup lang="ts">
import type { NewsInterface } from '@/interfaces/news-interface';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import dayjs from '@/lib/dayjs';
import { Head, Link } from '@inertiajs/vue3';
import { route } from 'ziggy-js';

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

    <div class="min-h-screen py-10 md:py-16">
        <div class="mx-auto max-w-3xl px-4 sm:px-6 lg:px-8">
            <!-- Back link -->
            <Link
                :href="route('news.index')"
                class="mb-8 inline-flex items-center gap-1.5 text-sm font-medium text-gray-500 transition-colors hover:text-[#054A29]"
            >
                <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                </svg>
                Назад кон новости
            </Link>

            <article>
                <!-- Hero image -->
                <div class="relative mb-10 aspect-video w-full overflow-hidden rounded-2xl bg-gray-100 shadow-lg">
                    <img :src="news.main_image_url" :alt="news.title" class="h-full w-full object-cover" />
                </div>

                <!-- Article header -->
                <header class="mb-10">
                    <time :datetime="news.created_at" class="mb-4 block text-xs font-semibold tracking-widest text-[#054A29] uppercase">
                        {{ dayjs(news.created_at).format('D MMMM YYYY') }}
                    </time>
                    <h1 class="mb-5 text-3xl leading-tight font-bold tracking-tight text-gray-900 md:text-4xl lg:text-[44px]">
                        {{ news.title }}
                    </h1>
                    <p class="text-lg leading-relaxed text-gray-500 md:text-xl">
                        {{ news.short_description }}
                    </p>
                    <div class="mt-8 border-t border-gray-100"></div>
                </header>

                <!-- Article body -->
                <div class="prose-content text-gray-700" v-html="news.content"></div>
            </article>
        </div>
    </div>
</template>

<style scoped>
.prose-content :deep(img) {
    margin-top: 2.5rem;
    margin-bottom: 2.5rem;
    border-radius: 1rem;
    width: 100%;
    box-shadow:
        0 10px 15px -3px rgb(0 0 0 / 0.1),
        0 4px 6px -4px rgb(0 0 0 / 0.1);
}

.prose-content :deep(p) {
    margin-bottom: 1.75rem;
    line-height: 1.85;
    font-size: 1.0625rem;
    color: #374151;
}

.prose-content :deep(h1),
.prose-content :deep(h2),
.prose-content :deep(h3),
.prose-content :deep(h4) {
    margin-top: 2.5rem;
    margin-bottom: 1.25rem;
    font-weight: 700;
    color: #111827;
    line-height: 1.3;
    letter-spacing: -0.02em;
}

.prose-content :deep(h1) {
    font-size: 2rem;
    margin-top: 3rem;
}

.prose-content :deep(h2) {
    font-size: 1.625rem;
    margin-top: 2.5rem;
}

.prose-content :deep(h3) {
    font-size: 1.375rem;
}

.prose-content :deep(h4) {
    font-size: 1.125rem;
}

.prose-content :deep(ul),
.prose-content :deep(ol) {
    margin-top: 1.5rem;
    margin-bottom: 1.5rem;
    padding-left: 1.75rem;
}

.prose-content :deep(li) {
    margin-bottom: 0.625rem;
    line-height: 1.7;
    color: #374151;
}

.prose-content :deep(a) {
    color: #054a29;
    text-decoration: underline;
    text-underline-offset: 3px;
    font-weight: 500;
    transition: color 0.2s;
}

.prose-content :deep(a:hover) {
    color: #033d22;
}

.prose-content :deep(blockquote) {
    border-left: 4px solid #054a29;
    padding-left: 1.5rem;
    padding-top: 0.25rem;
    padding-bottom: 0.25rem;
    margin: 2rem 0;
    font-style: italic;
    color: #4b5563;
    background-color: #f9fafb;
    border-radius: 0 0.5rem 0.5rem 0;
}

.prose-content :deep(code) {
    background-color: #f3f4f6;
    padding: 0.2rem 0.45rem;
    border-radius: 0.375rem;
    font-size: 0.875em;
    font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, 'Liberation Mono', monospace;
    color: #1f2937;
}

.prose-content :deep(pre) {
    background-color: #1f2937;
    color: #f9fafb;
    padding: 1.5rem;
    border-radius: 0.75rem;
    overflow-x: auto;
    margin: 2rem 0;
}

.prose-content :deep(pre code) {
    background-color: transparent;
    padding: 0;
    color: inherit;
}

.prose-content :deep(hr) {
    border: none;
    border-top: 1px solid #e5e7eb;
    margin: 3rem 0;
}
</style>
