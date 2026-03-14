<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
</script>

<template>
    <AppSection id="about-adrijan">
        <AppContainer>
            <div class="grid grid-cols-1 items-center gap-12 lg:grid-cols-2">
                <div class="order-1 flex flex-col gap-6 lg:order-first">
                    <div>
                        <p class="mb-2 text-xs font-semibold tracking-widest text-[#054A29] uppercase">За Адријан</p>
                        <h3 class="text-3xl font-bold tracking-tight text-gray-900 lg:text-[40px] lg:leading-[1.15]">Кој е Адријан Живков?</h3>
                    </div>
                    <div class="flex max-w-[480px] flex-col gap-5">
                        <blockquote class="border-l-4 border-[#054A29] py-1 pl-5">
                            <p class="text-base leading-relaxed font-medium text-gray-700 italic lg:text-[17px]">
                                „Во тие кобни моменти, Адријан херојски и без двоумење го жртвуваше сопствениот живот, обидувајќи се да спаси што е
                                можно поголем број луѓе."
                            </p>
                        </blockquote>
                        <p class="text-sm leading-relaxed text-gray-600 lg:text-base">
                            Адријан Живков е еден од трагично загинатите во пожарот во дискотеката „Пулс" во Кочани на 16 март 2025 година, во
                            најголемата трагедија што ја погоди Македонија, во која животот го загубија 63 лица.
                        </p>
                        <p class="text-sm leading-relaxed text-gray-600 lg:text-base">
                            Во чест на неговата храброст, човечност и пожртвуваност, ја основавме Фондацијата „Адријан Живков". Нашата мисија е да
                            пружиме поддршка и помош од секаков вид на сите оние на кои им е потребна, водејќи се од вредностите што Адријан ги
                            живееше и ги остави: солидарност, емпатија и несебичност.
                        </p>
                    </div>
                </div>
                <div class="order-first lg:order-last">
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
            </div>
        </AppContainer>
    </AppSection>
</template>

<style scoped></style>
