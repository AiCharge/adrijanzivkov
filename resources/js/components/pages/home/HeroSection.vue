<script setup lang="ts">
import HomeController from '@/actions/App/Http/Controllers/HomeController';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppNavbar from '@/components/layouts/AppNavbar.vue';
import { Link } from '@inertiajs/vue3';
</script>

<template>
    <div class="relative m-3 overflow-hidden rounded-2xl bg-[#054A29]">
        <!-- Subtle decorative circles -->
        <div class="pointer-events-none absolute top-0 right-0 h-80 w-80 translate-x-1/3 -translate-y-1/3 rounded-full bg-white/5"></div>
        <div class="pointer-events-none absolute bottom-0 left-0 h-48 w-48 -translate-x-1/3 translate-y-1/3 rounded-full bg-white/5"></div>

        <AppNavbar />

        <AppContainer class="relative pb-12 md:pb-16">
            <div class="grid grid-cols-1 items-center gap-10 pt-10 md:grid-cols-2 md:pt-14">
                <div class="flex max-w-[520px] flex-col gap-6">
                    <!-- Badge -->
                    <div class="inline-flex w-fit items-center gap-2 rounded-full border border-white/20 bg-white/10 px-3 py-1.5">
                        <div class="h-1.5 w-1.5 rounded-full bg-emerald-300"></div>
                        <span class="text-xs font-semibold tracking-widest text-white/80 uppercase">Фондација Адријан Живков</span>
                    </div>

                    <h1 class="text-[36px] leading-[1.15] font-bold tracking-tight text-white lg:text-[52px]">
                        Да внесеме топлина таму каде што е студено.
                    </h1>

                    <p class="max-w-[420px] text-base leading-relaxed font-normal text-white/70 lg:text-lg">
                        Нашата мисија е да донесеме надеж и поддршка таму каде што најмногу недостига – кај луѓе што се соочуваат со болест, неправда
                        или тежок животен предизвик.
                    </p>

                    <div class="flex flex-wrap items-center gap-4">
                        <Link
                            :href="HomeController.url() + '#donations'"
                            class="inline-flex cursor-pointer items-center gap-2 rounded-xl bg-white px-6 py-3 text-sm font-semibold text-[#054A29] shadow-lg shadow-black/10 transition-all duration-200 hover:bg-white/90 hover:shadow-xl"
                        >
                            Донирај
                            <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                            </svg>
                        </Link>
                        <Link
                            :href="HomeController.url() + '#about-adrijan'"
                            class="text-sm font-medium text-white/70 transition-colors hover:text-white"
                        >
                            Дознај повеќе →
                        </Link>
                    </div>
                </div>

                <div class="flex justify-center lg:justify-end">
                    <img src="/images/pages/home/hero-section/hero-image.webp" alt="" class="w-full max-w-[520px]" />
                </div>
            </div>
        </AppContainer>
    </div>
</template>

<style scoped></style>
