<script setup lang="ts">
import { index as newsIndex } from '@/actions/App/Http/Controllers/NewsController';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import type { NewsInterface } from '@/interfaces/news-interface';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import dayjs from '@/lib/dayjs';
import { Head, Link } from '@inertiajs/vue3';
import { computed } from 'vue';

interface Props {
    news: NewsInterface;
}

const props = defineProps<Props>();

const processedContent = computed(() => {
    if (!props.news.content) return '';
    if (typeof document === 'undefined') return props.news.content;

    const doc = new DOMParser().parseFromString(props.news.content, 'text/html');
    const children = Array.from(doc.body.children);

    const isImageBlock = (el: Element): boolean => {
        if (el.tagName === 'IMG') return true;
        if (el.tagName === 'FIGURE' && el.querySelector('img')) return true;
        if ((el.tagName === 'P' || el.tagName === 'DIV') && el.children.length === 1 && el.children[0].tagName === 'IMG') return true;
        return false;
    };

    let result = '';
    let i = 0;

    while (i < children.length) {
        const el = children[i];
        if (isImageBlock(el)) {
            const group: Element[] = [el];
            let j = i + 1;
            while (j < children.length && isImageBlock(children[j])) {
                group.push(children[j]);
                j++;
            }
            if (group.length >= 2) {
                result += '<div class="image-grid-2col">' + group.map((g) => g.outerHTML).join('') + '</div>';
            } else {
                result += el.outerHTML;
            }
            i = j;
        } else {
            result += el.outerHTML;
            i++;
        }
    }

    return result;
});

defineOptions({
    layout: DefaultLayout,
});
</script>

<template>
    <Head :title="news.title" />

    <div class="min-h-screen py-10 md:py-16">
        <AppContainer>
            <!-- Back link -->
            <Link
                :href="newsIndex.url()"
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
                    <div class="mt-8 border-t border-gray-100"></div>
                </header>

                <!-- Article body -->
                <div class="prose-content text-gray-700" v-html="processedContent"></div>
            </article>
        </AppContainer>
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

.prose-content :deep(.image-grid-2col) {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1rem;
    margin-top: 2.5rem;
    margin-bottom: 2.5rem;
}

@media (min-width: 768px) {
    .prose-content :deep(.image-grid-2col) {
        grid-template-columns: repeat(2, 1fr);
    }
}

.prose-content :deep(.image-grid-2col > p),
.prose-content :deep(.image-grid-2col > figure),
.prose-content :deep(.image-grid-2col > div) {
    margin: 0;
    overflow: hidden;
    border-radius: 1rem;
}

.prose-content :deep(.image-grid-2col img) {
    margin-top: 0;
    margin-bottom: 0;
    border-radius: 1rem;
    aspect-ratio: 16 / 9;
    object-fit: cover;
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
