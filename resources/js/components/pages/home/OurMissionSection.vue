<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
</script>

<template>
    <AppSection id="our-mission" class="bg-white">
        <AppContainer>
            <div class="grid grid-cols-1 items-center gap-12 lg:grid-cols-2">
                <div>
                    <img src="/images/pages/home/our-mission-section/our-mission.webp" alt="Helping hands" class="w-full rounded-2xl" />
                </div>
                <div class="flex flex-col gap-6">
                    <div>
                        <p class="mb-2 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Нашата мисија</p>
                        <h3 class="text-3xl font-bold tracking-tight text-gray-900 lg:text-[40px] lg:leading-[1.15]">Нашата мисија</h3>
                    </div>
                    <div class="flex flex-col gap-5">
                        <div class="flex gap-4">
                            <div class="mt-0.5 flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-[#054A29]/10">
                                <svg class="h-4 w-4 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                                    />
                                </svg>
                            </div>
                            <p class="text-sm leading-relaxed text-gray-600 lg:text-base">
                                Мисијата на Фондацијата „Адријан Живков" е да создаде трајна поддршка за поединци и семејства кои се соочуваат со
                                тешки животни околности. Преку хуманитарни активности, директна помош и општествено ангажирање, се стремиме да бидеме
                                извор на надеж.
                            </p>
                        </div>
                        <div class="flex gap-4">
                            <div class="mt-0.5 flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-[#054A29]/10">
                                <svg class="h-4 w-4 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"
                                    />
                                </svg>
                            </div>
                            <p class="text-sm leading-relaxed text-gray-600 lg:text-base">
                                Водени од вредностите што Адријан ги живееше — солидарност, емпатија и несебичност — нашата цел е да изградиме
                                заедница која дејствува со одговорност и грижа.
                            </p>
                        </div>
                        <div class="flex gap-4">
                            <div class="mt-0.5 flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-[#054A29]/10">
                                <svg class="h-4 w-4 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                                    />
                                </svg>
                            </div>
                            <p class="text-sm leading-relaxed text-gray-600 lg:text-base">
                                Нашата мисија е насочена кон создавање трајно позитивно влијание, преку конкретни дела кои оставаат значење денес и
                                надеж за иднината.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </AppContainer>
    </AppSection>
</template>

<style scoped></style>
