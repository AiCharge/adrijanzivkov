<script setup lang="ts">
import { index as newsIndex } from '@/actions/App/Http/Controllers/NewsController';
import NewsCard from '@/components/base/cards/NewsCard.vue';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import SectionHeading from '@/components/base/headings/SectionHeading.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
import type { NewsInterface } from '@/interfaces/news-interface';
import { Link } from '@inertiajs/vue3';

interface Props {
    news?: NewsInterface[];
}

defineProps<Props>();
</script>

<template>
    <AppSection id="news">
        <AppContainer>
            <SectionHeading
                v-reveal
                eyebrow="Актуелно"
                title="Новости"
                link-label="Сите новости"
                :link-href="newsIndex.url()"
                class="mb-10 md:mb-12"
            />

            <div v-if="news && news.length > 0" class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                <NewsCard v-for="(item, index) in news" :key="item.id" v-reveal="index * 100" :news="item" />
            </div>

            <div v-if="news && news.length > 0" class="mt-8 md:hidden">
                <Link
                    :href="newsIndex.url()"
                    class="flex items-center justify-center rounded-full border border-gray-300 bg-white px-6 py-3.5 text-sm font-semibold text-gray-900 transition-colors hover:border-primary hover:text-primary"
                >
                    Сите новости
                </Link>
            </div>

            <div v-else class="flex flex-col items-center justify-center rounded-2xl border border-gray-200 bg-white py-16 text-center">
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
