<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import LogoDark from '@/components/logos/LogoDark.vue';
import LogoLight from '@/components/logos/LogoLight.vue';
import { NavbarVariant } from '@/enums/navbar-variant';
import { Link } from '@inertiajs/vue3';
import { ref } from 'vue';

interface Props {
    variant?: NavbarVariant;
}

withDefaults(defineProps<Props>(), {
    variant: NavbarVariant.Light,
});

const isMenuOpen = ref(false);

const toggleMenu = () => {
    isMenuOpen.value = !isMenuOpen.value;
};
</script>

<template>
    <nav :class="variant === NavbarVariant.Dark ? 'bg-white' : ''" class="rounded-[16px] py-5">
        <AppContainer class="flex items-center justify-between rounded-[16px]">
            <Link href="/">
                <LogoLight v-if="variant === NavbarVariant.Light" />
                <LogoDark v-else />
            </Link>

            <button
                @click="toggleMenu"
                :class="variant === NavbarVariant.Light ? 'text-white' : 'text-[#1E1E1E]'"
                class="focus:outline-none md:hidden"
                aria-label="Toggle menu"
            >
                <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path v-if="!isMenuOpen" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                    <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
            </button>

            <div class="hidden items-center gap-10 md:flex">
                <Link href="/#about-adrijan" :class="variant === NavbarVariant.Light ? 'text-white' : 'text-[#1E1E1E]'" class="text-[18px] font-[400]"
                    >За Адријан</Link
                >
                <Link href="/#our-mission" :class="variant === NavbarVariant.Light ? 'text-white' : 'text-[#1E1E1E]'" class="text-[18px] font-[400]"
                    >Нашата мисија</Link
                >
                <Link href="/#contact" :class="variant === NavbarVariant.Light ? 'text-white' : 'text-[#1E1E1E]'" class="text-[18px] font-[400]"
                    >Контакт</Link
                >
            </div>
            <Link
                href="/#donations"
                :class="variant === NavbarVariant.Light ? 'bg-white text-[#161A1D]' : 'bg-[#054A29] text-white'"
                class="hidden cursor-pointer rounded-[8px] px-4 py-1 text-[18px] font-[500] md:block"
            >
                Донирај
            </Link>

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
                        <Link
                            href="/#about-adrijan"
                            @click="toggleMenu"
                            :class="variant === NavbarVariant.Light ? 'text-[#161A1D]' : 'text-[#1E1E1E]'"
                            class="py-2 text-[18px] font-[400] transition-opacity hover:opacity-75"
                        >
                            За Адријан
                        </Link>
                        <Link
                            href="/#our-mission"
                            @click="toggleMenu"
                            :class="variant === NavbarVariant.Light ? 'text-[#161A1D]' : 'text-[#1E1E1E]'"
                            class="py-2 text-[18px] font-[400] transition-opacity hover:opacity-75"
                        >
                            Нашата мисија
                        </Link>
                        <Link
                            href="/#contact"
                            @click="toggleMenu"
                            :class="variant === NavbarVariant.Light ? 'text-[#161A1D]' : 'text-[#1E1E1E]'"
                            class="py-2 text-[18px] font-[400] transition-opacity hover:opacity-75"
                        >
                            Контакт
                        </Link>
                        <Link
                            href="/#donations"
                            @click="toggleMenu"
                            :class="variant === NavbarVariant.Light ? 'bg-[#054A29]' : 'bg-[#054A29]'"
                            class="rounded-[8px] px-4 py-2 text-center text-[18px] font-[500] text-white transition-opacity hover:opacity-90"
                        >
                            Донирај
                        </Link>
                    </div>
                </div>
            </transition>
        </AppContainer>
    </nav>
</template>

<style scoped>
nav {
    position: relative;
}
</style>
