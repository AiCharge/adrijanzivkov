<script setup lang="ts">
import AppNavbar from '@/components/layouts/AppNavbar.vue';
import { NavbarVariant } from '@/enums/navbar-variant';
</script>

<template>
    <div class="min-h-screen">
        <div class="p-3 pb-0">
            <AppNavbar :variant="NavbarVariant.Dark" />
        </div>
        <slot />
    </div>
</template>

<style scoped></style>
