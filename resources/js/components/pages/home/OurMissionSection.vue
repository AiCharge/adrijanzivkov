<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
</script>

<template>
    <AppSection>
        <AppContainer>
            <div class="grid grid-cols-1 items-center gap-10 lg:grid-cols-2">
                <div>
                    <img src="/images/pages/home/our-mission-section/our-mission.webp" alt="Helping hands" class="w-fit" />
                </div>
                <div class="flex flex-col gap-4">
                    <h3 class="text-[28px] font-bold tracking-[0] text-[#1E1E1E] lg:text-[40px]">Наша мисија</h3>
                    <div class="flex max-w-[460px] flex-col gap-4">
                        <p class="text-[16px] leading-[20px] font-[400] lg:text-[18px] lg:leading-[100%]">
                            Мисијата на Фондацијата „Адријан Живков“ е да создаде трајна поддршка за поединци и семејства кои се соочуваат со тешки
                            животни околности. Преку хуманитарни активности, директна помош и општествено ангажирање, се стремиме да бидеме извор на
                            надеж таму каде што е најпотребна.
                        </p>
                        <p class="text-[16px] leading-[20px] font-[400] lg:text-[18px] lg:leading-[100%]">
                            Водени од вредностите што Адријан ги живееше, солидарност, емпатија и несебичност, нашата цел е да изградиме заедница која
                            не се свртува настрана, туку дејствува со одговорност и грижа.
                        </p>
                        <p class="text-[16px] leading-[20px] font-[400] lg:text-[18px] lg:leading-[100%]">
                            Нашата мисија е насочена кон создавање трајно позитивно влијание, преку конкретни дела кои оставаат значење денес и надеж
                            за иднината.
                        </p>
                    </div>
                </div>
            </div>
        </AppContainer>
    </AppSection>
</template>

<style scoped></style>
