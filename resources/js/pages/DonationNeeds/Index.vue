<script setup lang="ts">
import DonationNeedCard from '@/components/base/cards/DonationNeedCard.vue';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppPagination from '@/components/base/navigation/AppPagination.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
import PageHero from '@/components/base/sections/PageHero.vue';
import type { DonationNeedInterface } from '@/interfaces/donation-need-interface';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import { Head } from '@inertiajs/vue3';

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
</script>

<template>
    <Head title="Потреба за донации" />

    <PageHero eyebrow="Помогни" title="Потреба за донации" />

    <AppSection>
        <AppContainer>
            <div v-if="donationNeeds.data.length > 0">
                <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                    <DonationNeedCard v-for="(item, index) in donationNeeds.data" :key="item.id" v-reveal="(index % 3) * 100" :donation-need="item" />
                </div>

                <AppPagination v-if="donationNeeds.last_page > 1" :links="donationNeeds.links" />
            </div>

            <div v-else class="flex flex-col items-center justify-center gap-4 rounded-2xl border border-gray-200 bg-white px-6 py-24 text-center">
                <div class="flex h-14 w-14 items-center justify-center rounded-full bg-primary-light">
                    <svg class="h-7 w-7 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="1.5"
                            d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                        />
                    </svg>
                </div>
                <p class="text-sm font-medium text-gray-500">Моментално нема активни потреби за донации.</p>
            </div>
        </AppContainer>
    </AppSection>
</template>
