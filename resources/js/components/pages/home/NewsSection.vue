<script setup lang="ts">
import { index as newsIndex, show as newsShow } from '@/actions/App/Http/Controllers/NewsController';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
import type { NewsInterface } from '@/interfaces/news-interface';
import dayjs from '@/lib/dayjs';
import { Link } from '@inertiajs/vue3';

interface Props {
    news?: NewsInterface[];
}

defineProps<Props>();
</script>

<template>
    <AppSection id="news">
        <AppContainer>
            <div class="mb-10 flex items-end justify-between">
                <div>
                    <p class="mb-1.5 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Актуелно</p>
                    <h2 class="text-3xl font-bold tracking-tight text-gray-900 md:text-4xl">Новости</h2>
                </div>
                <Link
                    :href="newsIndex.url()"
                    class="hidden items-center gap-1.5 text-sm font-medium text-[#054A29] transition-colors hover:text-[#033d22] sm:flex"
                >
                    Сите новости
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                    </svg>
                </Link>
            </div>

            <div v-if="news && news.length > 0" class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                <Link
                    v-for="item in news"
                    :key="item.id"
                    :href="newsShow.url({ news: item.slug })"
                    class="group flex flex-col overflow-hidden rounded-2xl border border-gray-200 bg-white shadow-sm transition-all duration-300 hover:-translate-y-1 hover:border-gray-300 hover:shadow-lg"
                >
                    <div class="relative w-full overflow-hidden bg-gray-100" style="padding-bottom: 56.25%">
                        <img
                            :src="item.main_image_url || ''"
                            :alt="item.title"
                            class="absolute inset-0 h-full w-full object-cover transition-transform duration-500 group-hover:scale-105"
                        />
                    </div>
                    <div class="flex flex-1 flex-col p-5">
                        <div class="mb-3 text-xs font-medium text-gray-400">
                            {{ dayjs(item.created_at).format('D MMMM YYYY') }}
                        </div>
                        <h3 class="mb-3 line-clamp-2 text-base leading-snug font-semibold text-gray-900 transition-colors group-hover:text-[#054A29]">
                            {{ item.title }}
                        </h3>
                        <p class="mb-4 line-clamp-3 flex-1 text-sm leading-relaxed text-gray-500">
                            {{ item.short_description }}
                        </p>
                        <div class="flex items-center gap-1 text-xs font-semibold text-[#054A29]">
                            Прочитај повеќе
                            <svg
                                class="h-3.5 w-3.5 transition-transform duration-200 group-hover:translate-x-0.5"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                            </svg>
                        </div>
                    </div>
                </Link>
            </div>

            <!-- Mobile "all news" button -->
            <div v-if="news && news.length > 0" class="mt-8 sm:hidden">
                <Link
                    :href="newsIndex.url()"
                    class="flex items-center justify-center gap-2 rounded-xl border border-gray-200 bg-white px-6 py-3 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50"
                >
                    Сите новости
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                    </svg>
                </Link>
            </div>

            <!-- Empty state -->
            <div v-if="!news || news.length === 0" class="flex flex-col items-center justify-center py-16 text-center">
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
        </AppContainer>
    </AppSection>
</template>

<style scoped></style>
