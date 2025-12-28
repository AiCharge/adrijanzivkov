<script setup lang="ts">
import ContactSection from '@/components/pages/home/ContactSection.vue';
import HeroSection from '@/components/pages/home/HeroSection.vue';
import NewsSection from '@/components/pages/home/NewsSection.vue';
import OurMissionSection from '@/components/pages/home/OurMissionSection.vue';
import WhoIsAdrijanSection from '@/components/pages/home/WhoIsAdrijanSection.vue';
import { Head } from '@inertiajs/vue3';

interface News {
    id: number;
    title: string;
    short_description: string;
    main_image: string;
    created_at: string;
}

interface Props {
    latestNews?: News[];
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
