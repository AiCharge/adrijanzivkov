<script setup lang="ts">
import { show as newsShow } from '@/actions/App/Http/Controllers/NewsController';
import type { NewsInterface } from '@/interfaces/news-interface';
import dayjs from '@/lib/dayjs';
import { Link } from '@inertiajs/vue3';

interface Props {
    news: NewsInterface;
}

defineProps<Props>();
</script>

<template>
    <Link
        :href="newsShow.url({ news: news.slug })"
        class="group flex h-full flex-col overflow-hidden rounded-2xl border border-gray-200 bg-white transition-colors duration-300 hover:border-primary/40"
    >
        <div class="relative aspect-video overflow-hidden bg-gray-100">
            <img
                :src="news.main_image_url || ''"
                :alt="news.title"
                class="absolute inset-0 h-full w-full object-cover transition-transform duration-700 ease-(--ease-soft) group-hover:scale-[1.03]"
            />
        </div>
        <div class="flex flex-1 flex-col gap-3 p-6">
            <span class="text-xs font-medium text-gray-500">{{ dayjs(news.created_at).format('D MMMM YYYY') }}</span>
            <h3 class="line-clamp-2 text-lg leading-snug font-bold text-gray-900 transition-colors duration-300 group-hover:text-primary">
                {{ news.title }}
            </h3>
            <p class="line-clamp-3 flex-1 text-sm leading-relaxed text-gray-600">{{ news.short_description }}</p>
            <span class="inline-flex items-center gap-2 pt-1 text-sm font-semibold text-primary">
                Прочитај повеќе
                <svg
                    class="h-4 w-4 transition-transform duration-300 group-hover:translate-x-1"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14m-6-6 6 6-6 6" />
                </svg>
            </span>
        </div>
    </Link>
</template>
