<script setup lang="ts">
import AppFooter from '@/components/layouts/AppFooter.vue';
import AppNavbar from '@/components/layouts/AppNavbar.vue';
import { NavbarVariant } from '@/enums/navbar-variant';
</script>

<template>
    <div class="flex min-h-screen flex-col">
        <div class="sticky top-0 z-40 px-3 pt-3">
            <AppNavbar :variant="NavbarVariant.Dark" />
        </div>
        <main class="grow">
            <slot />
        </main>
        <AppFooter />
    </div>
</template>
