<script setup lang="ts">
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import type { DonationNeedInterface } from '@/interfaces/donation-need-interface';
import { Head, Link, router } from '@inertiajs/vue3';
import { route } from 'ziggy-js';

interface PaginationLink {
    url: string | null;
    label: string;
    active: boolean;
}

interface PaginatedDonationNeeds {
    data: DonationNeedInterface[];
    links: PaginationLink[];
    current_page: number;
    last_page: number;
    per_page: number;
    total: number;
    from: number | null;
    to: number | null;
}

interface Props {
    donationNeeds: PaginatedDonationNeeds;
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
    <Head title="Потреба за донации" />

    <div class="min-h-screen py-10 md:py-16">
        <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
            <!-- Page header -->
            <div class="mb-12">
                <p class="mb-2 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Помогни</p>
                <h1 class="text-3xl font-bold tracking-tight text-gray-900 md:text-4xl">Потреба за донации</h1>
            </div>

            <div v-if="donationNeeds.data && donationNeeds.data.length > 0">
                <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                    <div
                        v-for="item in donationNeeds.data"
                        :key="item.id"
                        class="flex flex-col overflow-hidden rounded-2xl border border-gray-200 bg-white shadow-sm transition-all duration-300 hover:-translate-y-1 hover:border-gray-300 hover:shadow-lg"
                    >
                        <!-- Image -->
                        <div class="relative w-full overflow-hidden bg-gray-100" style="padding-bottom: 62%">
                            <img
                                :src="item.image_url || ''"
                                :alt="item.title"
                                class="absolute inset-0 h-full w-full object-cover transition-transform duration-500"
                            />
                        </div>

                        <!-- Content -->
                        <div class="flex flex-1 flex-col p-5">
                            <!-- Amount badge -->
                            <div class="mb-3 inline-flex w-fit items-center gap-1.5 rounded-full bg-[#054A29]/10 px-3 py-1">
                                <svg class="h-3.5 w-3.5 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                                <span class="text-xs font-semibold text-[#054A29]">{{ formatAmount(item.needed_amount) }} МКД</span>
                            </div>

                            <h3 class="mb-2 text-base font-semibold leading-snug text-gray-900">{{ item.title }}</h3>
                            <p class="mb-5 line-clamp-3 flex-1 text-sm leading-relaxed text-gray-500">{{ item.description }}</p>

                            <Link
                                :href="route('donation-needs.show', { donation_need: item.id })"
                                class="flex items-center justify-center gap-2 rounded-xl bg-[#054A29] px-4 py-2.5 text-sm font-semibold text-white shadow-sm transition-all duration-200 hover:bg-[#033d22] hover:shadow-md"
                            >
                                Донирај
                                <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                                </svg>
                            </Link>
                        </div>
                    </div>
                </div>

                <!-- Pagination -->
                <nav v-if="donationNeeds.last_page > 1" class="mt-12 flex items-center justify-center">
                    <div class="flex gap-1.5">
                        <button
                            v-for="link in donationNeeds.links"
                            :key="link.label"
                            @click="link.url ? router.visit(link.url) : null"
                            :disabled="!link.url"
                            :class="[
                                'rounded-xl px-4 py-2 text-sm font-medium transition-all duration-200',
                                link.active
                                    ? 'bg-[#054A29] text-white shadow-sm'
                                    : link.url
                                      ? 'border border-gray-200 bg-white text-gray-700 hover:border-gray-300 hover:bg-gray-50'
                                      : 'cursor-not-allowed border border-gray-100 bg-gray-50 text-gray-300',
                            ]"
                            v-html="link.label"
                        />
                    </div>
                </nav>
            </div>

            <!-- Empty state -->
            <div v-else class="flex flex-col items-center justify-center py-24 text-center">
                <div class="mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-gray-100">
                    <svg class="h-7 w-7 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                    </svg>
                </div>
                <p class="text-sm font-medium text-gray-500">Моментално нема активни потреби за донации.</p>
            </div>
        </div>
    </div>
</template>

<style scoped></style>
