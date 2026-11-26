<script setup lang="ts">
import AboutController from '@/actions/App/Http/Controllers/AboutController';
import { index as donationNeedsIndex } from '@/actions/App/Http/Controllers/DonationNeedController';
import HomeController from '@/actions/App/Http/Controllers/HomeController';
import { index as newsIndex } from '@/actions/App/Http/Controllers/NewsController';
import { create as storyCreate } from '@/actions/App/Http/Controllers/StorySubmissionController';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import LogoDark from '@/components/logos/LogoDark.vue';
import LogoLight from '@/components/logos/LogoLight.vue';
import { NavbarVariant } from '@/enums/navbar-variant';
import { Link } from '@inertiajs/vue3';
import { computed, ref } from 'vue';

interface Props {
    variant?: NavbarVariant;
}

const props = withDefaults(defineProps<Props>(), {
    variant: NavbarVariant.Light,
});

const isLight = computed(() => props.variant === NavbarVariant.Light);

const links = [
    { label: 'За фондацијата', href: AboutController.url() },
    { label: 'Нашата мисија', href: HomeController.url() + '#our-mission' },
    { label: 'Новости', href: newsIndex.url() },
    { label: 'Донации', href: donationNeedsIndex.url() },
    { label: 'Контакт', href: HomeController.url() + '#contact' },
];

const isMenuOpen = ref(false);

const toggleMenu = () => {
    isMenuOpen.value = !isMenuOpen.value;
};
</script>

<template>
    <nav :class="isLight ? '' : 'rounded-2xl border border-gray-200 bg-white'" class="relative z-40 py-4">
        <AppContainer class="flex items-center justify-between gap-6">
            <Link :href="HomeController.url()" class="shrink-0">
                <LogoLight v-if="isLight" />
                <LogoDark v-else />
            </Link>

            <div class="hidden items-center gap-8 xl:flex">
                <Link
                    v-for="link in links"
                    :key="link.label"
                    :href="link.href"
                    :class="isLight ? 'text-white/75 hover:text-white' : 'text-gray-600 hover:text-primary'"
                    class="text-sm font-medium whitespace-nowrap transition-colors duration-200"
                >
                    {{ link.label }}
                </Link>
            </div>

            <div class="hidden shrink-0 items-center gap-3 xl:flex">
                <Link
                    :href="storyCreate.url()"
                    :class="
                        isLight
                            ? 'border-white/30 text-white hover:bg-white/10'
                            : 'border-gray-300 text-gray-900 hover:border-primary hover:text-primary'
                    "
                    class="rounded-full border px-5 py-2.5 text-sm font-semibold whitespace-nowrap transition-colors duration-200"
                >
                    Побарај помош
                </Link>
                <Link
                    :href="HomeController.url() + '#donations'"
                    :class="isLight ? 'bg-white text-primary hover:bg-primary-light' : 'bg-primary text-white hover:bg-primary-dark'"
                    class="rounded-full px-5 py-2.5 text-sm font-semibold whitespace-nowrap transition-colors duration-200"
                >
                    Донирај
                </Link>
            </div>

            <button
                @click="toggleMenu"
                :class="isLight ? 'border-white/30 text-white hover:bg-white/10' : 'border-gray-200 text-gray-700 hover:bg-gray-100'"
                class="rounded-full border p-2.5 transition-colors focus:outline-none xl:hidden"
                :aria-expanded="isMenuOpen"
                aria-label="Toggle menu"
            >
                <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path v-if="!isMenuOpen" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7h16M4 12h16M4 17h16" />
                    <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
            </button>

            <transition
                enter-active-class="transition ease-out duration-200"
                enter-from-class="opacity-0 -translate-y-1"
                enter-to-class="opacity-100 translate-y-0"
                leave-active-class="transition ease-in duration-150"
                leave-from-class="opacity-100 translate-y-0"
                leave-to-class="opacity-0 -translate-y-1"
            >
                <div v-if="isMenuOpen" class="absolute top-full right-3 left-3 z-50 mt-2 rounded-2xl border border-gray-200 bg-white p-3 xl:hidden">
                    <div class="flex flex-col">
                        <Link
                            v-for="link in links"
                            :key="link.label"
                            :href="link.href"
                            @click="toggleMenu"
                            class="rounded-xl px-4 py-3 text-base font-medium text-gray-700 transition-colors hover:bg-primary-light hover:text-primary"
                        >
                            {{ link.label }}
                        </Link>
                        <Link
                            :href="storyCreate.url()"
                            @click="toggleMenu"
                            class="rounded-xl px-4 py-3 text-base font-semibold text-primary transition-colors hover:bg-primary-light"
                        >
                            Побарај помош
                        </Link>
                    </div>
                    <div class="mt-2 border-t border-gray-100 pt-3">
                        <Link
                            :href="HomeController.url() + '#donations'"
                            @click="toggleMenu"
                            class="block rounded-full bg-primary px-4 py-3 text-center text-sm font-semibold text-white transition-colors hover:bg-primary-dark"
                        >
                            Донирај
                        </Link>
                    </div>
                </div>
            </transition>
        </AppContainer>
    </nav>
</template>
