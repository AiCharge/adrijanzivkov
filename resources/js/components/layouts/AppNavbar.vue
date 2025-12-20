<script setup lang="ts">
import LogoLight from '@/components/logos/LogoLight.vue';
import { Link } from '@inertiajs/vue3';
</script>

<template>
    <nav class="flex items-center justify-between rounded-[16px]">
        <Link href="/">
            <LogoLight />
        </Link>
        <div class="flex items-center gap-10">
            <Link class="text-[18px] font-[400] text-white">За Адријан</Link>
            <Link class="text-[18px] font-[400] text-white">Нашата мисија</Link>
            <Link class="text-[18px] font-[400] text-white">Контакт</Link>
        </div>
        <a class="rounded-[8px] bg-white px-4 py-1 text-[18px] font-[500] text-[#161A1D]"> Донирај </a>
    </nav>
</template>

<style scoped></style>
