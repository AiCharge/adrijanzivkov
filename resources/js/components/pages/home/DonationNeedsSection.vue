<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
import type { DonationNeedInterface } from '@/interfaces/donation-need-interface';
import { Link } from '@inertiajs/vue3';
import { route } from 'ziggy-js';

interface Props {
    donationNeeds?: DonationNeedInterface[];
}

defineProps<Props>();

function formatAmount(amount: number): string {
    return new Intl.NumberFormat('mk-MK').format(amount);
}

function stripHtml(html: string): string {
    const div = document.createElement('div');
    div.innerHTML = html;
    return div.textContent || div.innerText || '';
}
</script>

<template>
    <AppSection id="donations">
        <AppContainer>
            <div class="mb-10 flex items-end justify-between">
                <div>
                    <p class="mb-1.5 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Помогни</p>
                    <h2 class="text-3xl font-bold tracking-tight text-gray-900 md:text-4xl">Потреба за донации</h2>
                </div>
                <Link
                    :href="route('donation-needs.index')"
                    class="hidden items-center gap-1.5 text-sm font-medium text-[#054A29] transition-colors hover:text-[#033d22] sm:flex"
                >
                    Сите потреби
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                    </svg>
                </Link>
            </div>

            <div v-if="donationNeeds && donationNeeds.length > 0" class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                <div
                    v-for="item in donationNeeds"
                    :key="item.id"
                    class="flex flex-col overflow-hidden rounded-2xl border border-gray-200 bg-white shadow-sm"
                >
                    <!-- Image -->
                    <div class="relative w-full overflow-hidden bg-gray-100" style="padding-bottom: 62%">
                        <img :src="item.image_url || ''" :alt="item.title" class="absolute inset-0 h-full w-full object-cover" />
                    </div>

                    <!-- Content -->
                    <div class="flex flex-1 flex-col p-5">
                        <!-- Amount badge -->
                        <div class="mb-3 inline-flex w-fit items-center gap-1.5 rounded-full bg-[#054A29]/10 px-3 py-1">
                            <svg class="h-3.5 w-3.5 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                />
                            </svg>
                            <span class="text-xs font-semibold text-[#054A29]">{{ formatAmount(item.needed_amount) }} МКД</span>
                        </div>

                        <h3 class="mb-2 text-base leading-snug font-semibold text-gray-900">{{ item.title }}</h3>
                        <p class="mb-5 line-clamp-3 flex-1 text-sm leading-relaxed text-gray-500">{{ stripHtml(item.description) }}</p>

                        <Link
                            :href="route('donation-needs.show', { donation_need: item.id })"
                            class="flex items-center justify-center gap-2 rounded-xl bg-[#054A29] px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition-all duration-200 hover:bg-[#033d22] hover:shadow-md"
                        >
                            Донирај
                            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                                />
                            </svg>
                        </Link>
                    </div>
                </div>
            </div>

            <!-- Mobile "all needs" button -->
            <div v-if="donationNeeds && donationNeeds.length > 0" class="mt-8 sm:hidden">
                <Link
                    :href="route('donation-needs.index')"
                    class="flex items-center justify-center gap-2 rounded-xl border border-gray-200 bg-white px-6 py-3 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50"
                >
                    Сите потреби
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                    </svg>
                </Link>
            </div>

            <!-- Empty state -->
            <div v-else class="flex flex-col items-center justify-center py-16 text-center">
                <div class="mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-gray-100">
                    <svg class="h-7 w-7 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="1.5"
                            d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                        />
                    </svg>
                </div>
                <p class="text-sm font-medium text-gray-500">Моментално нема активни донациски потреби.</p>
            </div>
        </AppContainer>
    </AppSection>
</template>

<style scoped></style>
