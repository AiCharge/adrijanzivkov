<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import SectionHeading from '@/components/base/headings/SectionHeading.vue';
import AppSection from '@/components/base/sections/AppSection.vue';

const pillars = [
    {
        text: 'Мисијата на Фондацијата „Адријан Живков" е да создаде трајна поддршка за поединци и семејства кои се соочуваат со тешки животни околности. Преку хуманитарни активности, директна помош и општествено ангажирање, се стремиме да бидеме извор на надеж.',
        icon: 'M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z',
    },
    {
        text: 'Водени од вредностите на солидарност, емпатија и несебичност кои Адријан ги живееше, нашата цел е да изградиме заедница која дејствува со одговорност и грижа.',
        icon: 'M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z',
    },
    {
        text: 'Нашата мисија е насочена кон создавање трајно позитивно влијание, преку конкретни дела кои оставаат значење денес и надеж за иднината.',
        icon: 'M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z',
    },
];
</script>

<template>
    <AppSection id="our-mission" class="bg-white">
        <AppContainer>
            <SectionHeading v-reveal eyebrow="Нашата мисија" title="Нашата мисија" class="mb-10 md:mb-12" />

            <div class="grid grid-cols-1 items-start gap-6 lg:grid-cols-12 lg:gap-8">
                <div v-reveal class="overflow-hidden rounded-2xl lg:col-span-5">
                    <img src="/images/pages/home/our-mission-section/our-mission.webp" alt="Helping hands" class="w-full" />
                </div>

                <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:col-span-7 lg:gap-6">
                    <div
                        v-for="(pillar, index) in pillars"
                        :key="pillar.icon"
                        v-reveal="index * 100"
                        class="flex flex-col gap-5 rounded-2xl border border-gray-200 bg-white p-6 sm:p-7"
                    >
                        <span class="flex h-11 w-11 items-center justify-center rounded-xl bg-primary-light text-primary">
                            <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" :d="pillar.icon" />
                            </svg>
                        </span>
                        <p class="text-sm leading-relaxed text-gray-600 lg:text-base">{{ pillar.text }}</p>
                    </div>

                    <div v-reveal="300" class="flex flex-col gap-5 rounded-2xl bg-primary p-6 sm:p-7">
                        <span class="flex h-11 w-11 items-center justify-center rounded-xl bg-white/10 text-white">
                            <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                />
                            </svg>
                        </span>
                        <p class="text-sm leading-relaxed text-white/80 lg:text-base">
                            Фондацијата е финансирана исклучиво преку доброволни донации од пријателите и семејството на Адријан.
                            <strong class="font-semibold text-white">Не прифаќаме донации од трети лица.</strong>
                        </p>
                    </div>
                </div>
            </div>
        </AppContainer>
    </AppSection>
</template>
