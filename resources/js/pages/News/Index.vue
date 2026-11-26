<script setup lang="ts">
import NewsCard from '@/components/base/cards/NewsCard.vue';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppPagination from '@/components/base/navigation/AppPagination.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
import PageHero from '@/components/base/sections/PageHero.vue';
import type { NewsInterface } from '@/interfaces/news-interface';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import { Head } from '@inertiajs/vue3';

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

    <PageHero eyebrow="Актуелно" title="Новости" />

    <AppSection>
        <AppContainer>
            <div v-if="news.data && news.data.length > 0">
                <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                    <NewsCard v-for="(item, index) in news.data" :key="item.id" v-reveal="(index % 3) * 100" :news="item" />
                </div>

                <AppPagination v-if="news.last_page > 1" :links="news.links" />
            </div>

            <div v-else class="flex flex-col items-center justify-center rounded-2xl border border-gray-200 bg-white py-24 text-center">
                <div class="mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-primary-light">
                    <svg class="h-7 w-7 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
        </AppContainer>
    </AppSection>
</template>
