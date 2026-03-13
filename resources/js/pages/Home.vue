<script setup lang="ts">
import AppFooter from '@/components/layouts/AppFooter.vue';
import ContactSection from '@/components/pages/home/ContactSection.vue';
import DonationNeedsSection from '@/components/pages/home/DonationNeedsSection.vue';
import HeroSection from '@/components/pages/home/HeroSection.vue';
import NewsSection from '@/components/pages/home/NewsSection.vue';
import OurMissionSection from '@/components/pages/home/OurMissionSection.vue';
import WhoIsAdrijanSection from '@/components/pages/home/WhoIsAdrijanSection.vue';
import type { DonationNeedInterface } from '@/interfaces/donation-need-interface';
import type { NewsInterface } from '@/interfaces/news-interface';
import { Head } from '@inertiajs/vue3';

interface Props {
    latestNews?: NewsInterface[];
    donationNeeds?: DonationNeedInterface[];
}

defineProps<Props>();
</script>

<template>
    <Head title="Почетна" />
    <Toaster richColors />

    <div>
        <HeroSection />
        <NewsSection :news="latestNews" />
        <WhoIsAdrijanSection />
        <OurMissionSection />
        <DonationNeedsSection :donation-needs="donationNeeds" />
        <ContactSection />
        <AppFooter />
    </div>
</template>
