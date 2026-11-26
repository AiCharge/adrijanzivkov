<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';

interface Props {
    eyebrow?: string;
    title: string;
    description?: string;
}

withDefaults(defineProps<Props>(), {
    eyebrow: undefined,
    description: undefined,
});
</script>

<template>
    <div class="m-3 rounded-3xl bg-primary">
        <AppContainer class="py-14 md:py-20 lg:py-24">
            <div :class="$slots.aside ? 'lg:grid-cols-12' : ''" class="grid grid-cols-1 items-center gap-10 lg:gap-16">
                <div :class="$slots.aside ? 'lg:col-span-7' : 'max-w-3xl'" class="flex flex-col gap-6">
                    <p v-if="eyebrow" class="animate-rise text-xs font-semibold tracking-[0.2em] text-white/60 uppercase">
                        {{ eyebrow }}
                    </p>
                    <h1
                        class="animate-rise text-4xl leading-[1.05] font-bold tracking-tight text-balance text-white [animation-delay:80ms] md:text-5xl lg:text-6xl"
                    >
                        {{ title }}
                    </h1>
                    <p
                        v-if="description"
                        class="max-w-2xl animate-rise text-base leading-relaxed text-pretty text-white/70 [animation-delay:160ms] lg:text-lg"
                    >
                        {{ description }}
                    </p>
                    <div v-if="$slots.default" class="animate-rise [animation-delay:240ms]">
                        <slot />
                    </div>
                </div>

                <div v-if="$slots.aside" class="animate-rise [animation-delay:200ms] lg:col-span-5">
                    <slot name="aside" />
                </div>
            </div>
        </AppContainer>
    </div>
</template>
