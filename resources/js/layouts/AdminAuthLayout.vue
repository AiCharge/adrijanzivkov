<script setup lang="ts">
import LogoLight from '@/components/logos/LogoLight.vue';
import { Toaster } from '@/components/ui/sonner';
import 'vue-sonner/style.css';

/**
 * The administrator's sign-in page: one card in the middle of the screen, with the foundation's mark on top.
 */
defineProps<{
    title: string;
    description?: string;
}>();
</script>

<template>
    <div class="relative isolate flex min-h-svh flex-col items-center justify-center bg-[var(--brand-forest)] px-4 py-10">
        <!-- A soft green glow behind the card. -->
        <div
            aria-hidden="true"
            class="pointer-events-none absolute inset-0 -z-10 bg-[radial-gradient(60%_50%_at_50%_0%,color-mix(in_oklch,var(--brand-green)_55%,transparent),transparent)]"
        />

        <div class="mb-8 flex flex-col items-center gap-2">
            <LogoLight class="h-9 w-auto" aria-label="Фондација Адријан Живков" />
            <span class="text-[10px] font-semibold tracking-[0.22em] text-emerald-200 uppercase">Администрација</span>
        </div>

        <main class="bg-card w-full max-w-sm rounded-lg border p-8 shadow-2xl shadow-black/30">
            <header class="mb-7 space-y-1.5 text-center">
                <h1 class="text-2xl font-semibold tracking-tight">{{ title }}</h1>
                <p v-if="description" class="text-muted-foreground text-sm">{{ description }}</p>
            </header>

            <slot />
        </main>

        <p class="mt-6 text-xs text-white/50">Фондација „Адријан Живков" · Администрација</p>

        <Toaster />
    </div>
</template>
