<script setup lang="ts">
import ContactSection from '@/components/pages/home/ContactSection.vue';
import HeroSection from '@/components/pages/home/HeroSection.vue';
import NewsSection from '@/components/pages/home/NewsSection.vue';
import OurMissionSection from '@/components/pages/home/OurMissionSection.vue';
import WhoIsAdrijanSection from '@/components/pages/home/WhoIsAdrijanSection.vue';
import type { NewsInterface } from '@/interfaces/news-interface';
import { Head } from '@inertiajs/vue3';

interface Props {
    latestNews?: NewsInterface[];
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
        <ContactSection />
    </div>
</template>
