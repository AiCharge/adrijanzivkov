<script setup lang="ts">
import HomeController from '@/actions/App/Http/Controllers/HomeController';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import type { DonationNeedInterface } from '@/interfaces/donation-need-interface';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import { formatAmount } from '@/lib/format';
import { Head, Link } from '@inertiajs/vue3';

interface Props {
    donationNeed: DonationNeedInterface;
}

defineProps<Props>();

defineOptions({
    layout: DefaultLayout,
});
</script>

<template>
    <Head :title="donationNeed.title" />

    <div class="py-10 md:py-16">
        <AppContainer>
            <Link
                :href="HomeController.url() + '#donations'"
                class="group inline-flex animate-rise items-center gap-2 text-sm font-semibold text-gray-600 transition-colors hover:text-primary"
            >
                <svg
                    class="h-4 w-4 transition-transform duration-300 group-hover:-translate-x-1"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 12H5m6 6-6-6 6-6" />
                </svg>
                Назад кон донации
            </Link>

            <div class="mt-10 grid grid-cols-1 gap-10 lg:grid-cols-12 lg:items-start lg:gap-12">
                <div class="order-2 flex flex-col gap-8 lg:order-1 lg:col-span-7">
                    <div class="animate-rise overflow-hidden rounded-2xl bg-gray-100 [animation-delay:80ms]">
                        <img :src="donationNeed.image_url || ''" :alt="donationNeed.title" class="w-full animate-zoom-out object-cover" />
                    </div>

                    <div class="flex flex-col gap-4">
                        <div class="inline-flex w-fit items-center gap-1.5 rounded-full bg-primary-light px-3 py-1">
                            <svg class="h-3.5 w-3.5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                />
                            </svg>
                            <span class="text-xs font-semibold text-primary">Потребен износ: {{ formatAmount(donationNeed.needed_amount) }} МКД</span>
                        </div>
                        <h1 class="text-3xl leading-[1.1] font-bold tracking-tight text-balance text-gray-900 md:text-4xl lg:text-5xl">
                            {{ donationNeed.title }}
                        </h1>
                    </div>

                    <div v-reveal class="border-t border-gray-200 pt-8">
                        <p class="mb-6 text-xs font-semibold tracking-[0.2em] text-gray-400 uppercase">За оваа потреба</p>
                        <div class="prose-content" v-html="donationNeed.description"></div>
                    </div>
                </div>

                <aside class="order-1 animate-rise [animation-delay:160ms] lg:sticky lg:top-28 lg:order-2 lg:col-span-5">
                    <div class="overflow-hidden rounded-2xl border border-gray-200 bg-white">
                        <div class="bg-primary px-7 py-7">
                            <p class="text-xs font-semibold tracking-[0.2em] text-white/60 uppercase">Потребен износ</p>
                            <p class="mt-2 text-4xl font-bold tracking-tight text-white">
                                {{ formatAmount(donationNeed.needed_amount) }}
                                <span class="text-xl font-semibold text-white/70">МКД</span>
                            </p>
                        </div>

                        <div class="p-7">
                            <div v-if="donationNeed.sticky_content" class="prose-content prose-content-sm" v-html="donationNeed.sticky_content"></div>

                            <p v-else class="rounded-xl bg-primary-light px-4 py-3 text-sm leading-relaxed text-gray-700">
                                Секоја донација, колку и да е мала, прави разлика. Благодариме на вашата поддршка и несебичност.
                            </p>
                        </div>
                    </div>
                </aside>
            </div>
        </AppContainer>
    </div>
</template>
