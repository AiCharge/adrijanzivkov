<script setup lang="ts">
import HomeController from '@/actions/App/Http/Controllers/HomeController';
import { create as storyCreate } from '@/actions/App/Http/Controllers/StorySubmissionController';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppNavbar from '@/components/layouts/AppNavbar.vue';
import { Link } from '@inertiajs/vue3';
</script>

<template>
    <div class="m-3 rounded-3xl bg-primary">
        <AppNavbar />

        <AppContainer class="pt-10 pb-12 md:pt-16 md:pb-16 lg:pt-20 lg:pb-20">
            <div class="grid grid-cols-1 items-center gap-12 lg:grid-cols-12 lg:gap-16">
                <div class="flex flex-col gap-7 lg:col-span-6">
                    <p class="animate-rise text-xs font-semibold tracking-[0.2em] text-white/60 uppercase">Фондација Адријан Живков</p>

                    <h1
                        class="animate-rise text-[40px] leading-[1.05] font-bold tracking-tight text-balance text-white [animation-delay:80ms] sm:text-5xl lg:text-[64px]"
                    >
                        Да внесеме топлина таму каде што е студено.
                    </h1>

                    <p class="max-w-[480px] animate-rise text-base leading-relaxed text-white/70 [animation-delay:160ms] lg:text-lg">
                        Нашата мисија е да донесеме надеж и поддршка таму каде што најмногу недостига, кај луѓе што се соочуваат со болест, неправда
                        или тежок животен предизвик.
                    </p>

                    <div class="flex animate-rise flex-wrap items-center gap-3 pt-2 [animation-delay:240ms]">
                        <Link
                            :href="HomeController.url() + '#donations'"
                            class="group inline-flex items-center gap-2 rounded-full bg-white px-7 py-3.5 text-sm font-semibold text-primary transition-colors duration-200 hover:bg-primary-light"
                        >
                            Донирај
                            <svg
                                class="h-4 w-4 transition-transform duration-300 group-hover:translate-x-1"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14m-6-6 6 6-6 6" />
                            </svg>
                        </Link>
                        <Link
                            :href="storyCreate.url()"
                            class="inline-flex items-center rounded-full border border-white/30 px-7 py-3.5 text-sm font-semibold text-white transition-colors duration-200 hover:bg-white/10"
                        >
                            Побарај помош
                        </Link>
                    </div>
                </div>

                <div class="flex animate-rise justify-center [animation-delay:200ms] lg:col-span-6 lg:justify-end">
                    <img src="/images/pages/home/hero-section/hero-image.webp" alt="" class="w-full max-w-[520px] animate-zoom-out" />
                </div>
            </div>
        </AppContainer>
    </div>
</template>
