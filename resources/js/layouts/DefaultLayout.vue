<script setup lang="ts">
import AppFooter from '@/components/layouts/AppFooter.vue';
import AppNavbar from '@/components/layouts/AppNavbar.vue';
import { NavbarVariant } from '@/enums/navbar-variant';
</script>

<template>
    <div class="flex min-h-screen flex-col">
        <div class="p-3 pb-0">
            <AppNavbar :variant="NavbarVariant.Dark" />
        </div>
        <main class="grow">
            <slot />
        </main>
        <AppFooter />
    </div>
</template>

<style scoped></style>
