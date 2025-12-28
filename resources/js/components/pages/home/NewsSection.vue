<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
import type { NewsInterface } from '@/interfaces/news-interface';
import dayjs from '@/lib/dayjs';
import { Link } from '@inertiajs/vue3';
import { route } from 'ziggy-js';

interface Props {
    news?: NewsInterface[];
}

defineProps<Props>();
</script>

<template>
    <AppSection id="news">
        <AppContainer>
            <h2 class="mb-12 text-3xl font-bold text-gray-900 md:text-4xl">Новости</h2>
            <div v-if="news && news.length > 0" class="grid grid-cols-1 gap-8 md:grid-cols-2 lg:grid-cols-3">
                <Link
                    v-for="item in news"
                    :key="item.id"
                    :href="route('news.show', { news: item.slug })"
                    class="group flex flex-col overflow-hidden rounded-lg bg-white transition-transform duration-200 hover:scale-[1.02]"
                >
                    <div class="relative w-full overflow-hidden bg-gray-100" style="padding-bottom: 56.25%">
                        <img
                            :src="item.main_image_url || ''"
                            :alt="item.title"
                            class="absolute inset-0 h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
                        />
                    </div>
                    <div class="flex flex-1 flex-col p-5">
                        <h3 class="mb-3 line-clamp-2 text-lg leading-snug font-semibold text-gray-900 transition-colors group-hover:text-[#054A29]">
                            {{ item.title }}
                        </h3>
                        <p class="mb-4 line-clamp-3 flex-1 text-sm leading-relaxed text-gray-600">
                            {{ item.short_description }}
                        </p>
                        <div class="text-xs font-medium text-gray-500">
                            {{ dayjs(item.created_at).format('D MMMM YYYY') }}
                        </div>
                    </div>
                </Link>
            </div>
            <div v-else class="py-12 text-center text-gray-500">Нема достапни новости.</div>
        </AppContainer>
    </AppSection>
</template>

<style scoped></style>
