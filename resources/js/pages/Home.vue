<script setup lang="ts">
import HeroSection from '@/components/pages/home/HeroSection.vue';
import { Head } from '@inertiajs/vue3';
</script>

<template>
    <Head title="Почетна" />
    <Toaster richColors />

    <div>
        <HeroSection />
    </div>
</template>
