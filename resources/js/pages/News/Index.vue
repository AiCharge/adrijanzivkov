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

    <div class="min-h-screen py-8 md:py-12">
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
            <h1 class="mb-12 text-3xl font-bold text-gray-900 md:text-4xl">Новости</h1>

            <div v-if="news.data && news.data.length > 0" class="space-y-8">
                <div v-for="item in news.data" :key="item.id" class="overflow-hidden rounded-lg bg-white shadow-sm transition-shadow hover:shadow-md">
                    <Link :href="route('news.show', { news: item.slug })" class="block md:flex">
                        <div class="relative h-64 w-full overflow-hidden md:h-auto md:w-80 md:flex-shrink-0">
                            <img
                                :src="item.main_image_url || ''"
                                :alt="item.title"
                                class="h-full w-full object-cover transition-transform duration-300 hover:scale-105"
                            />
                        </div>
                        <div class="flex flex-1 flex-col p-6 md:p-8">
                            <div class="mb-3 text-sm font-medium text-gray-500">
                                {{ dayjs(item.created_at).format('D MMMM YYYY') }}
                            </div>
                            <h2 class="mb-4 text-2xl leading-tight font-bold text-gray-900 transition-colors hover:text-[#054A29] md:text-3xl">
                                {{ item.title }}
                            </h2>
                            <p class="mb-6 flex-1 text-base leading-relaxed text-gray-600 md:text-lg">
                                {{ item.short_description }}
                            </p>
                            <div class="text-sm font-medium text-[#054A29]">Прочитај повеќе →</div>
                        </div>
                    </Link>
                </div>

                <!-- Pagination -->
                <nav v-if="news.last_page > 1" class="mt-12 flex items-center justify-center">
                    <div class="flex gap-2">
                        <button
                            v-for="link in news.links"
                            :key="link.label"
                            @click="link.url ? router.visit(link.url) : null"
                            :disabled="!link.url"
                            :class="[
                                'rounded-lg px-4 py-2 text-sm font-medium transition-colors',
                                link.active
                                    ? 'bg-[#054A29] text-white'
                                    : link.url
                                      ? 'border border-gray-300 bg-white text-gray-700 hover:bg-gray-50'
                                      : 'cursor-not-allowed bg-gray-100 text-gray-400',
                            ]"
                            v-html="link.label"
                        />
                    </div>
                </nav>
            </div>

            <div v-else class="py-12 text-center text-gray-500">Нема достапни новости.</div>
        </div>
    </div>
</template>

<style scoped></style>


