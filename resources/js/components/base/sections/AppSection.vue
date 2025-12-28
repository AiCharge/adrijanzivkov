<script setup lang="ts">
interface Props {
    id?: string;
}

withDefaults(defineProps<Props>(), {
    id: undefined,
});
</script>

<template>
    <section :id="id" class="py-10 md:py-20">
        <slot />
    </section>
</template>

<style scoped></style>
