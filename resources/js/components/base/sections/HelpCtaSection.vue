<script setup lang="ts">
import { create as storyCreate } from '@/actions/App/Http/Controllers/StorySubmissionController';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import { Link } from '@inertiajs/vue3';
</script>

<template>
    <section class="px-3 pt-10 md:pt-16">
        <div v-reveal class="rounded-3xl bg-primary py-16 md:py-20">
            <AppContainer>
                <div class="mx-auto flex max-w-2xl flex-col items-center gap-6 text-center">
                    <h2 class="text-3xl leading-[1.1] font-bold tracking-tight text-balance text-white md:text-4xl lg:text-[44px]">
                        Имаш потреба од помош?
                    </h2>
                    <p class="text-base leading-relaxed text-white/70 md:text-lg">
                        Доколку поминуваш низ тежок период, сподели ја својата приказна. Ќе направиме сè што е во наша моќ да помогнеме.
                    </p>
                    <Link
                        :href="storyCreate.url()"
                        class="group mt-2 inline-flex items-center gap-2 rounded-full bg-white px-7 py-3.5 text-sm font-semibold text-primary transition-colors hover:bg-primary-light"
                    >
                        Побарај помош
                        <svg
                            class="h-4 w-4 transition-transform duration-300 group-hover:translate-x-1"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14m-6-6 6 6-6 6" />
                        </svg>
                    </Link>
                </div>
            </AppContainer>
        </div>
    </section>
</template>
