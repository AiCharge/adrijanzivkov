<script setup lang="ts">
import type { NewsInterface } from '@/interfaces/news-interface';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import dayjs from '@/lib/dayjs';
import { Head, Link, router } from '@inertiajs/vue3';
import { route } from 'ziggy-js';

interface PaginationLink {
    url: string | null;
    label: string;
    active: boolean;
}

interface PaginatedNews {
    data: NewsInterface[];
    links: PaginationLink[];
    current_page: number;
    last_page: number;
    per_page: number;
    total: number;
    from: number | null;
    to: number | null;
}

interface Props {
    news: PaginatedNews;
}

defineProps<Props>();

defineOptions({
    layout: DefaultLayout,
});
</script>

<template>
    <Head title="Новости" />

    <div class="min-h-screen py-10 md:py-16">
        <div class="mx-auto max-w-5xl px-4 sm:px-6 lg:px-8">
            <!-- Page header -->
            <div class="mb-12">
                <p class="mb-2 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Актуелно</p>
                <h1 class="text-3xl font-bold tracking-tight text-gray-900 md:text-4xl">Новости</h1>
            </div>

            <div v-if="news.data && news.data.length > 0" class="space-y-6">
                <Link
                    v-for="item in news.data"
                    :key="item.id"
                    :href="route('news.show', { news: item.slug })"
                    class="group flex overflow-hidden rounded-2xl border border-gray-200 bg-white shadow-sm transition-all duration-300 hover:border-gray-300 hover:shadow-md md:flex-row"
                >
                    <div class="relative h-56 w-full overflow-hidden bg-gray-100 md:h-auto md:w-72 md:shrink-0">
                        <img
                            :src="item.main_image_url || ''"
                            :alt="item.title"
                            class="h-full w-full object-cover transition-transform duration-500 group-hover:scale-105"
                        />
                    </div>
                    <div class="flex flex-1 flex-col justify-between p-6 md:p-8">
                        <div>
                            <div class="mb-3 text-xs font-medium text-gray-400">
                                {{ dayjs(item.created_at).format('D MMMM YYYY') }}
                            </div>
                            <h2 class="mb-3 text-xl leading-snug font-bold text-gray-900 transition-colors group-hover:text-[#054A29] md:text-2xl">
                                {{ item.title }}
                            </h2>
                            <p class="line-clamp-3 text-sm leading-relaxed text-gray-500 md:text-base">
                                {{ item.short_description }}
                            </p>
                        </div>
                        <div class="mt-5 flex items-center gap-1 text-sm font-semibold text-[#054A29]">
                            Прочитај повеќе
                            <svg
                                class="h-4 w-4 transition-transform duration-200 group-hover:translate-x-0.5"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                            </svg>
                        </div>
                    </div>
                </Link>

                <!-- Pagination -->
                <nav v-if="news.last_page > 1" class="mt-12 flex items-center justify-center">
                    <div class="flex gap-1.5">
                        <button
                            v-for="link in news.links"
                            :key="link.label"
                            @click="link.url ? router.visit(link.url) : null"
                            :disabled="!link.url"
                            :class="[
                                'rounded-xl px-4 py-2 text-sm font-medium transition-all duration-200',
                                link.active
                                    ? 'bg-[#054A29] text-white shadow-sm'
                                    : link.url
                                      ? 'border border-gray-200 bg-white text-gray-700 hover:border-gray-300 hover:bg-gray-50'
                                      : 'cursor-not-allowed border border-gray-100 bg-gray-50 text-gray-300',
                            ]"
                            v-html="link.label"
                        />
                    </div>
                </nav>
            </div>

            <!-- Empty state -->
            <div v-else class="flex flex-col items-center justify-center py-24 text-center">
                <div class="mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-gray-100">
                    <svg class="h-7 w-7 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="1.5"
                            d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2m-4-3H9M7 16h6M7 8h6v4H7V8z"
                        />
                    </svg>
                </div>
                <p class="text-sm font-medium text-gray-500">Нема достапни новости.</p>
            </div>
        </div>
    </div>
</template>

<style scoped></style>
