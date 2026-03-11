<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
</script>

<template>
    <AppSection id="contact" class="m-3 rounded-2xl bg-[#054A29]">
        <AppContainer class="text-white">
            <div class="mb-10">
                <p class="mb-2 text-xs font-semibold tracking-widest text-white/60 uppercase">Контакт</p>
                <h3 class="text-3xl font-bold tracking-tight lg:text-[40px]">Стапи во контакт со Фондацијата</h3>
                <p class="mt-4 max-w-[500px] text-base font-normal text-white/70 lg:text-lg">
                    Нашата мисија е да донесеме надеж и поддршка таму каде што најмногу недостига.
                </p>
            </div>

            <div class="flex flex-col gap-4 sm:flex-row sm:flex-wrap sm:items-stretch sm:gap-4">
                <a
                    href="mailto:adrijanfoundation@gmail.com"
                    class="flex items-center gap-4 rounded-2xl bg-white/10 px-5 py-4 transition-colors hover:bg-white/15 sm:flex-1"
                >
                    <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-white/15">
                        <svg class="h-5 w-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="1.75"
                                d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                            />
                        </svg>
                    </div>
                    <div>
                        <p class="mb-0.5 text-xs font-medium text-white/60">Е-пошта</p>
                        <p class="text-sm font-semibold text-white">adrijanfoundation@gmail.com</p>
                    </div>
                </a>

                <a
                    href="tel:+38977720475"
                    class="flex items-center gap-4 rounded-2xl bg-white/10 px-5 py-4 transition-colors hover:bg-white/15 sm:flex-1"
                >
                    <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-white/15">
                        <svg class="h-5 w-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="1.75"
                                d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"
                            />
                        </svg>
                    </div>
                    <div>
                        <p class="mb-0.5 text-xs font-medium text-white/60">Телефон</p>
                        <p class="text-sm font-semibold text-white">+389 77720475</p>
                    </div>
                </a>

                <a
                    href="https://www.instagram.com/adrijanzivkov/"
                    target="_blank"
                    class="flex items-center gap-4 rounded-2xl bg-white/10 px-5 py-4 transition-colors hover:bg-white/15 sm:flex-1"
                >
                    <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-white/15">
                        <svg class="h-5 w-5 text-white" fill="currentColor" viewBox="0 0 20 20">
                            <path
                                d="M13.2 10a3.2 3.2 0 11-6.4 0 3.2 3.2 0 016.4 0zM20 5.6v8.8A5.6 5.6 0 0114.4 20H5.6A5.6 5.6 0 010 14.4V5.6A5.6 5.6 0 015.6 0h8.8A5.6 5.6 0 0120 5.6zM14.8 10a4.8 4.8 0 10-9.6 0 4.8 4.8 0 009.6 0zm1.6-5.2a1.2 1.2 0 10-2.4 0 1.2 1.2 0 002.4 0z"
                            />
                        </svg>
                    </div>
                    <div>
                        <p class="mb-0.5 text-xs font-medium text-white/60">Инстаграм</p>
                        <p class="text-sm font-semibold text-white">@adrijanzivkov</p>
                    </div>
                </a>
            </div>
        </AppContainer>
    </AppSection>
</template>

<style scoped></style>
