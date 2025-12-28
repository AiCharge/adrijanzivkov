<script setup lang="ts">
import type { AppSectionPropsInterface } from '@/interfaces/AppSectionPropsInterface';

withDefaults(defineProps<AppSectionPropsInterface>(), {
    id: undefined,
});
</script>

<template>
    <section :id="id" class="py-10 md:py-20">
        <slot />
    </section>
</template>

<style scoped></style>
