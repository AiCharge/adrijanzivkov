<script setup lang="ts">
import AppNavbar from '@/components/layouts/AppNavbar.vue';
import { NavbarVariant } from '@/enums/navbar-variant';
</script>

<template>
    <div class="m-3">
        <AppNavbar :variant="NavbarVariant.Dark" />
        <slot />
    </div>
</template>

<style scoped></style>
