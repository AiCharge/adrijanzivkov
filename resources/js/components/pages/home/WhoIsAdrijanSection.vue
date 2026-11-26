<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
</script>

<template>
    <AppSection id="about-adrijan" class="bg-white">
        <AppContainer>
            <div class="grid grid-cols-1 items-center gap-12 lg:grid-cols-12 lg:gap-16">
                <div v-reveal class="lg:col-span-5">
                    <img
                        src="/images/pages/home/who-is-adrijan-section/adrijan-desktop.webp"
                        alt="Adrijan Zhivkov"
                        class="hidden w-full rounded-2xl lg:block"
                    />
                    <img
                        src="/images/pages/home/who-is-adrijan-section/adrijan-mobile.webp"
                        alt="Adrijan Zhivkov"
                        class="w-full rounded-2xl lg:hidden"
                    />
                </div>

                <div v-reveal="120" class="flex flex-col gap-8 lg:col-span-7">
                    <div class="flex flex-col gap-4">
                        <p class="text-xs font-semibold tracking-[0.2em] text-primary uppercase">За Адријан</p>
                        <h2 class="text-3xl leading-[1.1] font-bold tracking-tight text-gray-900 md:text-4xl lg:text-[44px]">
                            Кој е Адријан Живков?
                        </h2>
                    </div>

                    <blockquote class="border-l-2 border-primary pl-6">
                        <p class="text-lg leading-relaxed font-medium text-gray-900 italic md:text-xl">
                            „Во тие кобни моменти, Адријан херојски и без двоумење го жртвуваше сопствениот живот, обидувајќи се да спаси што е можно
                            поголем број луѓе."
                        </p>
                    </blockquote>

                    <div class="flex flex-col gap-5 text-base leading-relaxed text-gray-600">
                        <p>
                            Адријан Живков е еден од трагично загинатите во пожарот во дискотеката „Пулс" во Кочани на 16 март 2025 година, во
                            најголемата трагедија што ја погоди Македонија, во која животот го загубија 63 лица.
                        </p>
                        <p>
                            Во чест на неговата храброст, човечност и пожртвуваност, ја основавме Фондацијата „Адријан Живков". Нашата мисија е да
                            пружиме поддршка и помош од секаков вид на сите оние на кои им е потребна, водејќи се од вредностите што Адријан ги
                            живееше и ги остави: солидарност, емпатија и несебичност.
                        </p>
                    </div>
                </div>
            </div>
        </AppContainer>
    </AppSection>
</template>
