<script setup lang="ts">
import { show as donationShow } from '@/actions/App/Http/Controllers/DonationNeedController';
import type { DonationNeedInterface } from '@/interfaces/donation-need-interface';
import { formatAmount, stripHtml } from '@/lib/format';
import { Link } from '@inertiajs/vue3';

interface Props {
    donationNeed: DonationNeedInterface;
}

defineProps<Props>();
</script>

<template>
    <div
        class="group flex h-full flex-col overflow-hidden rounded-2xl border border-gray-200 bg-white transition-colors duration-300 hover:border-primary/40"
    >
        <div class="relative aspect-[100/62] overflow-hidden bg-gray-100">
            <img
                :src="donationNeed.image_url || ''"
                :alt="donationNeed.title"
                class="absolute inset-0 h-full w-full object-cover transition-transform duration-700 ease-(--ease-soft) group-hover:scale-[1.03]"
            />
        </div>

        <div class="flex flex-1 flex-col gap-3 p-6">
            <div class="inline-flex w-fit items-center gap-1.5 rounded-full bg-primary-light px-3 py-1">
                <svg class="h-3.5 w-3.5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    />
                </svg>
                <span class="text-xs font-semibold text-primary">{{ formatAmount(donationNeed.needed_amount) }} МКД</span>
            </div>

            <h3 class="text-lg leading-snug font-bold text-gray-900">{{ donationNeed.title }}</h3>
            <p class="line-clamp-3 flex-1 text-sm leading-relaxed text-gray-600">{{ stripHtml(donationNeed.description) }}</p>

            <Link
                :href="donationShow.url(donationNeed.id)"
                class="mt-3 flex items-center justify-center gap-2 rounded-full bg-primary px-4 py-3 text-sm font-semibold text-white transition-colors duration-300 hover:bg-primary-dark"
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
</template>
