<script setup lang="ts">
import { index as donationIndex } from '@/actions/App/Http/Controllers/DonationNeedController';
import DonationNeedCard from '@/components/base/cards/DonationNeedCard.vue';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import SectionHeading from '@/components/base/headings/SectionHeading.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
import type { DonationNeedInterface } from '@/interfaces/donation-need-interface';
import { Link } from '@inertiajs/vue3';

interface Props {
    donationNeeds?: DonationNeedInterface[];
}

defineProps<Props>();
</script>

<template>
    <AppSection id="donations">
        <AppContainer>
            <SectionHeading
                v-reveal
                eyebrow="Помогни"
                title="Потреба за донации"
                link-label="Сите потреби"
                :link-href="donationIndex.url()"
                class="mb-10 md:mb-12"
            />

            <div v-if="donationNeeds && donationNeeds.length > 0" class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                <DonationNeedCard v-for="(item, index) in donationNeeds" :key="item.id" v-reveal="(index % 3) * 100" :donation-need="item" />
            </div>

            <div v-if="donationNeeds && donationNeeds.length > 0" class="mt-8 md:hidden">
                <Link
                    :href="donationIndex.url()"
                    class="flex items-center justify-center rounded-full border border-gray-300 bg-white px-6 py-3.5 text-sm font-semibold text-gray-900 transition-colors hover:border-primary hover:text-primary"
                >
                    Сите потреби
                </Link>
            </div>

            <div v-else class="flex flex-col items-center justify-center rounded-2xl border border-gray-200 py-16 text-center">
                <div class="mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-primary-light">
                    <svg class="h-7 w-7 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
