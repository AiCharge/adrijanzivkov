<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppNavbar from '@/components/layouts/AppNavbar.vue';
</script>

<template>
    <div class="m-3 rounded-[16px] bg-[#054A29]">
        <AppContainer class="py-5">
            <AppNavbar />
            <div class="grid grid-cols-1 items-center gap-10 pt-12 pb-2 md:grid-cols-2">
                <div class="flex max-w-[500px] flex-col gap-5">
                    <h1 class="text-[32px] leading-[38px] font-bold tracking-[0] text-white lg:text-[44px] lg:leading-[44px]">
                        Да внесеме топлина таму каде што е студено.
                    </h1>
                    <p class="max-w-[400px] text-[16px] leading-[100%] font-[400] tracking-[0] text-white">
                        Нашата мисија е да донесеме надеж и поддршка таму каде што најмногу недостига – кај луѓе што се соочуваат со болест, неправда
                        или тежок животен предизвик.
                    </p>
                    <a class="inline-flex w-fit cursor-pointer rounded-[8px] bg-white px-4 py-1 text-[18px] font-[500] text-[#161A1D]"> Донирај </a>
                </div>
                <div>
                    <img src="/images/pages/home/hero-section/hero-image.webp" alt="" />
                </div>
            </div>
        </AppContainer>
    </div>
</template>

<style scoped></style>
