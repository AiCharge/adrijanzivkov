<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
</script>

<template>
    <AppSection id="about-adrijan">
        <AppContainer>
            <div class="grid grid-cols-1 items-center gap-10 lg:grid-cols-2">
                <div class="order-1 flex flex-col gap-4 lg:order-0">
                    <h3 class="text-[28px] font-bold tracking-[0] text-[#1E1E1E] lg:text-[40px]">Кој е Адријан Живков?</h3>
                    <div class="flex max-w-[460px] flex-col gap-4">
                        <p class="text-[16px] leading-[20px] font-[400] lg:text-[18px] lg:leading-[100%]">
                            Адријан Живков е еден од трагично загинатите во дискотеката „Пулс“, во најголемата трагедија што ја погоди Македонија, во
                            која животот го загубија 63 лица. Во тие кобни моменти, Адријан херојски и без двоумење го жртвуваше сопствениот живот,
                            обидувајќи се да спаси што е можно поголем број луѓе.
                        </p>
                        <p class="text-[16px] leading-[20px] font-[400] lg:text-[18px] lg:leading-[100%]">
                            Во чест на неговата храброст, човечност и пожртвуваност, ја основавме Фондацијата „Адријан Живков“. Нашата мисија е да
                            пружиме поддршка и помош од секаков вид на сите оние на кои им е потребна, водејќи се од вредностите што Адријан ги
                            живееше и ги остави како наследство - солидарност, емпатија и несебичност.
                        </p>
                    </div>
                </div>
                <div class="order-0 lg:order-1">
                    <img src="/images/pages/home/who-is-adrijan-section/adrijan-desktop.webp" alt="Adrijan Zhivkov" class="hidden w-fit lg:block" />
                    <img src="/images/pages/home/who-is-adrijan-section/adrijan-mobile.webp" alt="Adrijan Zhivkov" class="w-fit lg:hidden" />
                </div>
            </div>
        </AppContainer>
    </AppSection>
</template>

<style scoped></style>
