<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import LogoDark from '@/components/logos/LogoDark.vue';
import LogoLight from '@/components/logos/LogoLight.vue';
import HomeController from '@/actions/App/Http/Controllers/HomeController';
import { index as newsIndex } from '@/actions/App/Http/Controllers/NewsController';
import { create as storyCreate } from '@/actions/App/Http/Controllers/StorySubmissionController';
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
    <nav :class="variant === NavbarVariant.Dark ? 'border border-gray-100 bg-white shadow-sm' : ''" class="rounded-2xl py-4">
        <AppContainer class="flex items-center justify-between">
            <Link :href="route('index')">
                <LogoLight v-if="variant === NavbarVariant.Light" />
                <LogoDark v-else />
            </Link>

            <button
                @click="toggleMenu"
                :class="variant === NavbarVariant.Light ? 'text-white hover:bg-white/10' : 'text-gray-700 hover:bg-gray-100'"
                class="rounded-lg p-2 transition-colors focus:outline-none lg:hidden"
                aria-label="Toggle menu"
            >
                <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path v-if="!isMenuOpen" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                    <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
            </button>

            <div class="hidden items-center gap-8 lg:flex">
                <Link
                    :href="HomeController.url() + '#about-adrijan'"
                    :class="variant === NavbarVariant.Light ? 'text-white/80 hover:text-white' : 'text-gray-600 hover:text-[#054A29]'"
                    class="text-sm font-medium tracking-wide transition-colors duration-200"
                    >За Адријан</Link
                >
                <Link
                    :href="HomeController.url() + '#our-mission'"
                    :class="variant === NavbarVariant.Light ? 'text-white/80 hover:text-white' : 'text-gray-600 hover:text-[#054A29]'"
                    class="text-sm font-medium tracking-wide transition-colors duration-200"
                    >Нашата мисија</Link
                >
                <Link
                    :href="newsIndex.url()"
                    :class="variant === NavbarVariant.Light ? 'text-white/80 hover:text-white' : 'text-gray-600 hover:text-[#054A29]'"
                    class="text-sm font-medium tracking-wide transition-colors duration-200"
                    >Новости</Link
                >
                <Link
                    :href="HomeController.url() + '#contact'"
                    :class="variant === NavbarVariant.Light ? 'text-white/80 hover:text-white' : 'text-gray-600 hover:text-[#054A29]'"
                    class="text-sm font-medium tracking-wide transition-colors duration-200"
                    >Контакт</Link
                >
                <Link
                    :href="storyCreate.url()"
                    :class="
                        variant === NavbarVariant.Light
                            ? 'border-white/30 text-white hover:bg-white/10'
                            : 'border-[#054A29]/30 text-[#054A29] hover:bg-[#054A29]/5'
                    "
                    class="rounded-xl border px-4 py-1.5 text-sm font-semibold tracking-wide transition-colors duration-200"
                    >Побарај помош</Link
                >
            </div>

            <Link
                :href="HomeController.url() + '#donations'"
                :class="variant === NavbarVariant.Light ? 'bg-white text-[#054A29] hover:bg-white/90' : 'bg-[#054A29] text-white hover:bg-[#033d22]'"
                class="hidden cursor-pointer rounded-xl px-5 py-2 text-sm font-semibold shadow-sm transition-all duration-200 hover:shadow-md lg:block"
            >
                Донирај
            </Link>

            <transition
                enter-active-class="transition ease-out duration-200"
                enter-from-class="opacity-0 -translate-y-1"
                enter-to-class="opacity-100 translate-y-0"
                leave-active-class="transition ease-in duration-150"
                leave-from-class="opacity-100 translate-y-0"
                leave-to-class="opacity-0 -translate-y-1"
            >
                <div
                    v-if="isMenuOpen"
                    class="absolute top-full right-0 left-0 z-50 mt-2 rounded-2xl border border-gray-100 bg-white shadow-xl lg:hidden"
                >
                    <div class="flex flex-col p-3">
                        <Link
                            :href="HomeController.url() + '#about-adrijan'"
                            @click="toggleMenu"
                            class="rounded-xl px-4 py-3 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50 hover:text-[#054A29]"
                        >
                            За Адријан
                        </Link>
                        <Link
                            :href="HomeController.url() + '#our-mission'"
                            @click="toggleMenu"
                            class="rounded-xl px-4 py-3 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50 hover:text-[#054A29]"
                        >
                            Нашата мисија
                        </Link>
                        <Link
                            :href="newsIndex.url()"
                            @click="toggleMenu"
                            class="rounded-xl px-4 py-3 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50 hover:text-[#054A29]"
                        >
                            Новости
                        </Link>
                        <Link
                            :href="HomeController.url() + '#contact'"
                            @click="toggleMenu"
                            class="rounded-xl px-4 py-3 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50 hover:text-[#054A29]"
                        >
                            Контакт
                        </Link>
                        <Link
                            :href="storyCreate.url()"
                            @click="toggleMenu"
                            class="rounded-xl px-4 py-3 text-sm font-semibold text-[#054A29] transition-colors hover:bg-[#054A29]/5"
                        >
                            Побарај помош
                        </Link>
                        <div class="mt-2 border-t border-gray-100 pt-2">
                            <Link
                                :href="HomeController.url() + '#donations'"
                                @click="toggleMenu"
                                class="block rounded-xl bg-[#054A29] px-4 py-3 text-center text-sm font-semibold text-white transition-colors hover:bg-[#033d22]"
                            >
                                Донирај
                            </Link>
                        </div>
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
