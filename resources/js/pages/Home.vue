<script setup lang="ts">
import ContactSection from '@/components/pages/home/ContactSection.vue';
import HeroSection from '@/components/pages/home/HeroSection.vue';
import OurMissionSection from '@/components/pages/home/OurMissionSection.vue';
import WhoIsAdrijanSection from '@/components/pages/home/WhoIsAdrijanSection.vue';
import { Head } from '@inertiajs/vue3';
</script>

<template>
    <Head title="Почетна" />
    <Toaster richColors />

    <div>
        <HeroSection />
        <WhoIsAdrijanSection />
        <OurMissionSection />
        <ContactSection />
    </div>
</template>
