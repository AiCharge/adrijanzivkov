<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';

const contacts = [
    {
        label: 'Е-пошта',
        value: 'adrijanfoundation@gmail.com',
        href: 'mailto:adrijanfoundation@gmail.com',
        external: false,
        icon: 'M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z',
    },
    {
        label: 'Телефон',
        value: '+389 77720475',
        href: 'tel:+38977720475',
        external: false,
        icon: 'M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z',
    },
    {
        label: 'Инстаграм',
        value: '@adrijanzivkov',
        href: 'https://www.instagram.com/adrijanzivkov/',
        external: true,
        icon: 'M16 11.37A4 4 0 1112.63 8 4 4 0 0116 11.37zM17.5 6.5h.01M7 2h10a5 5 0 015 5v10a5 5 0 01-5 5H7a5 5 0 01-5-5V7a5 5 0 015-5z',
    },
];
</script>

<template>
    <section id="contact" class="px-3 pt-10 md:pt-16">
        <div v-reveal class="rounded-3xl bg-primary py-16 md:py-24">
            <AppContainer>
                <div class="grid grid-cols-1 gap-12 lg:grid-cols-12 lg:items-end">
                    <div class="flex flex-col gap-5 lg:col-span-6">
                        <p class="text-xs font-semibold tracking-[0.2em] text-white/60 uppercase">Контакт</p>
                        <h2 class="text-3xl leading-[1.1] font-bold tracking-tight text-balance text-white md:text-4xl lg:text-[44px]">
                            Стапи во контакт со Фондацијата
                        </h2>
                        <p class="max-w-md text-base leading-relaxed text-white/70 lg:text-lg">
                            Нашата мисија е да донесеме надеж и поддршка таму каде што најмногу недостига.
                        </p>
                    </div>

                    <div class="flex flex-col gap-3 lg:col-span-6 lg:col-start-7">
                        <a
                            v-for="contact in contacts"
                            :key="contact.label"
                            :href="contact.href"
                            :target="contact.external ? '_blank' : undefined"
                            class="group flex items-center gap-4 rounded-2xl border border-white/15 px-5 py-4 transition-colors duration-200 hover:bg-white/10"
                        >
                            <span class="flex h-11 w-11 shrink-0 items-center justify-center rounded-full bg-white/10 text-white">
                                <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.75" :d="contact.icon" />
                                </svg>
                            </span>
                            <span class="flex min-w-0 flex-1 flex-col">
                                <span class="text-xs font-medium text-white/60">{{ contact.label }}</span>
                                <span class="truncate text-sm font-semibold text-white md:text-base">{{ contact.value }}</span>
                            </span>
                            <svg
                                class="h-4 w-4 shrink-0 text-white/40 transition-all duration-300 group-hover:translate-x-1 group-hover:text-white"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14m-6-6 6 6-6 6" />
                            </svg>
                        </a>
                    </div>
                </div>
            </AppContainer>
        </div>
    </section>
</template>
