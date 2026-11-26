<script setup lang="ts">
import { Link } from '@inertiajs/vue3';

interface Props {
    eyebrow: string;
    title: string;
    description?: string;
    linkLabel?: string;
    linkHref?: string;
    centered?: boolean;
    inverted?: boolean;
}

withDefaults(defineProps<Props>(), {
    description: undefined,
    linkLabel: undefined,
    linkHref: undefined,
    centered: false,
    inverted: false,
});
</script>

<template>
    <div :class="centered ? 'items-center text-center' : 'md:flex-row md:items-end md:justify-between'" class="flex flex-col gap-6">
        <div :class="centered ? 'items-center' : ''" class="flex max-w-2xl flex-col gap-4">
            <p :class="inverted ? 'text-white/60' : 'text-primary'" class="text-xs font-semibold tracking-[0.2em] uppercase">
                {{ eyebrow }}
            </p>
            <h2
                :class="inverted ? 'text-white' : 'text-gray-900'"
                class="text-3xl leading-[1.1] font-bold tracking-tight text-balance md:text-4xl lg:text-[44px]"
            >
                {{ title }}
            </h2>
            <p v-if="description" :class="inverted ? 'text-white/70' : 'text-gray-600'" class="text-base leading-relaxed text-pretty lg:text-lg">
                {{ description }}
            </p>
        </div>

        <Link
            v-if="linkLabel && linkHref && !centered"
            :href="linkHref"
            class="group hidden shrink-0 items-center gap-2 text-sm font-semibold text-primary transition-colors hover:text-primary-dark md:inline-flex"
        >
            {{ linkLabel }}
            <svg class="h-4 w-4 transition-transform duration-300 group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14m-6-6 6 6-6 6" />
            </svg>
        </Link>
    </div>
</template>
