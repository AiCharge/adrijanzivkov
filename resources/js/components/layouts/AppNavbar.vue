<script setup lang="ts">
import LogoLight from '@/components/logos/LogoLight.vue';
import { Link } from '@inertiajs/vue3';
import { ref } from 'vue';

const isMenuOpen = ref(false);

const toggleMenu = () => {
    isMenuOpen.value = !isMenuOpen.value;
};
</script>

<template>
    <nav class="flex items-center justify-between rounded-[16px]">
        <Link href="/">
            <LogoLight />
        </Link>

        <button @click="toggleMenu" class="text-white focus:outline-none md:hidden" aria-label="Toggle menu">
            <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path v-if="!isMenuOpen" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
        </button>

        <div class="hidden items-center gap-10 md:flex">
            <Link class="text-[18px] font-[400] text-white">За Адријан</Link>
            <Link class="text-[18px] font-[400] text-white">Нашата мисија</Link>
            <Link class="text-[18px] font-[400] text-white">Контакт</Link>
        </div>
        <a class="hidden cursor-pointer rounded-[8px] bg-white px-4 py-1 text-[18px] font-[500] text-[#161A1D] md:block"> Донирај </a>

        <transition
            enter-active-class="transition ease-out duration-200"
            enter-from-class="opacity-0 translate-y-1"
            enter-to-class="opacity-100 translate-y-0"
            leave-active-class="transition ease-in duration-150"
            leave-from-class="opacity-100 translate-y-0"
            leave-to-class="opacity-0 translate-y-1"
        >
            <div v-if="isMenuOpen" class="absolute top-full right-0 left-0 z-50 mt-2 rounded-[16px] bg-white shadow-lg md:hidden">
                <div class="flex flex-col gap-4 p-4">
                    <Link @click="toggleMenu" class="py-2 text-[18px] font-[400] text-[#161A1D] transition-opacity hover:opacity-75">
                        За Адријан
                    </Link>
                    <Link @click="toggleMenu" class="py-2 text-[18px] font-[400] text-[#161A1D] transition-opacity hover:opacity-75">
                        Нашата мисија
                    </Link>
                    <Link @click="toggleMenu" class="py-2 text-[18px] font-[400] text-[#161A1D] transition-opacity hover:opacity-75"> Контакт </Link>
                    <a
                        @click="toggleMenu"
                        class="rounded-[8px] bg-[#054A29] px-4 py-2 text-center text-[18px] font-[500] text-white transition-opacity hover:opacity-90"
                    >
                        Донирај
                    </a>
                </div>
            </div>
        </transition>
    </nav>
</template>

<style scoped>
nav {
    position: relative;
}
</style>
