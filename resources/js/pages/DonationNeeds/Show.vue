<script setup lang="ts">
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import type { DonationNeedInterface } from '@/interfaces/donation-need-interface';
import { Head, Link } from '@inertiajs/vue3';
import { route } from 'ziggy-js';

interface Props {
    donationNeed: DonationNeedInterface;
}

defineProps<Props>();

defineOptions({
    layout: DefaultLayout,
});

function formatAmount(amount: number): string {
    return new Intl.NumberFormat('mk-MK').format(amount);
}
</script>

<template>
    <Head :title="donationNeed.title" />

    <div class="min-h-screen py-10 md:py-16">
        <div class="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8">
            <!-- Back link -->
            <Link
                :href="route('index') + '#donations'"
                class="mb-8 inline-flex items-center gap-1.5 text-sm font-medium text-gray-500 transition-colors hover:text-[#054A29]"
            >
                <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                </svg>
                Назад кон донации
            </Link>

            <div class="grid grid-cols-1 gap-10 lg:grid-cols-5 lg:items-start">
                <!-- Left: image + description -->
                <div class="lg:col-span-3">
                    <!-- Hero image -->
                    <div class="mb-8 overflow-hidden rounded-2xl bg-gray-100 shadow-lg">
                        <img
                            :src="donationNeed.image_url || ''"
                            :alt="donationNeed.title"
                            class="h-full w-full object-cover"
                        />
                    </div>

                    <!-- Title + amount -->
                    <div class="mb-6">
                        <div class="mb-3 inline-flex items-center gap-1.5 rounded-full bg-[#054A29]/10 px-3 py-1">
                            <svg class="h-3.5 w-3.5 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            <span class="text-xs font-semibold text-[#054A29]">Потребен износ: {{ formatAmount(donationNeed.needed_amount) }} МКД</span>
                        </div>
                        <h1 class="text-2xl font-bold tracking-tight text-gray-900 md:text-3xl lg:text-4xl">
                            {{ donationNeed.title }}
                        </h1>
                    </div>

                    <div class="border-t border-gray-100 pt-6">
                        <p class="mb-4 text-sm font-semibold uppercase tracking-widest text-gray-400">За оваа потреба</p>
                        <div class="prose-content text-gray-700" v-html="donationNeed.description"></div>
                    </div>
                </div>

                <!-- Right: donate CTA card (sticky) -->
                <div class="lg:col-span-2 lg:sticky lg:top-8">
                    <div class="overflow-hidden rounded-2xl border border-gray-200 bg-white shadow-sm">
                        <!-- Card header -->
                        <div class="bg-[#054A29] px-6 py-5">
                            <p class="text-xs font-semibold tracking-widest text-white/70 uppercase mb-1">Потребен износ</p>
                            <p class="text-3xl font-bold text-white">{{ formatAmount(donationNeed.needed_amount) }} <span class="text-xl font-semibold text-white/80">МКД</span></p>
                        </div>

                        <!-- Card body -->
                        <div class="p-6">
                            <div class="rounded-xl bg-gray-50 px-4 py-3">
                                <p class="text-xs leading-relaxed text-gray-500">
                                    Секоја донација, колку и да е мала, прави разлика. Благодариме на вашата поддршка и несебичност.
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
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

.prose-content :deep(h1) { font-size: 2rem; margin-top: 3rem; }
.prose-content :deep(h2) { font-size: 1.625rem; margin-top: 2.5rem; }
.prose-content :deep(h3) { font-size: 1.375rem; }
.prose-content :deep(h4) { font-size: 1.125rem; }

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

.prose-content :deep(a:hover) { color: #033d22; }

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

.prose-content :deep(hr) {
    border: none;
    border-top: 1px solid #e5e7eb;
    margin: 3rem 0;
}
</style>
