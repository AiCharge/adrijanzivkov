<script setup lang="ts">
import AboutController from '@/actions/App/Http/Controllers/AboutController';
import HomeController from '@/actions/App/Http/Controllers/HomeController';
import { privacy, terms } from '@/actions/App/Http/Controllers/LegalController';
import { index as newsIndex } from '@/actions/App/Http/Controllers/NewsController';
import { create as storyCreate } from '@/actions/App/Http/Controllers/StorySubmissionController';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import LogoDark from '@/components/logos/LogoDark.vue';
import { Link } from '@inertiajs/vue3';

const navigationLinks = [
    { label: 'За Адријан', href: HomeController.url() + '#about-adrijan' },
    { label: 'Нашата мисија', href: HomeController.url() + '#our-mission' },
    { label: 'За фондацијата', href: AboutController.url() },
    { label: 'Новости', href: newsIndex.url() },
    { label: 'Контакт', href: HomeController.url() + '#contact' },
];

const legalLinks = [
    { label: 'Услови на користење', href: terms.url() },
    { label: 'Политика за приватност', href: privacy.url() },
];
</script>

<template>
    <footer class="mt-10 border-t border-gray-200 bg-white md:mt-16">
        <AppContainer class="pt-14 pb-8 md:pt-16">
            <div class="grid grid-cols-1 gap-10 md:grid-cols-12">
                <div class="flex flex-col gap-5 md:col-span-6">
                    <Link :href="HomeController.url()" class="w-fit">
                        <LogoDark />
                    </Link>
                    <p class="max-w-xs text-sm leading-relaxed text-gray-600">
                        Фондација посветена на помош, надеж и инспирација за луѓе кои имаат потреба.
                    </p>
                </div>

                <div class="flex flex-col gap-4 md:col-span-3">
                    <p class="text-xs font-semibold tracking-[0.2em] text-gray-400 uppercase">Навигација</p>
                    <nav class="flex flex-col gap-3">
                        <Link
                            v-for="link in navigationLinks"
                            :key="link.label"
                            :href="link.href"
                            class="w-fit text-sm text-gray-600 transition-colors hover:text-primary"
                        >
                            {{ link.label }}
                        </Link>
                        <Link :href="storyCreate.url()" class="w-fit text-sm font-semibold text-primary transition-colors hover:text-primary-dark">
                            Побарај помош
                        </Link>
                    </nav>
                </div>

                <div class="flex flex-col gap-4 md:col-span-3">
                    <p class="text-xs font-semibold tracking-[0.2em] text-gray-400 uppercase">Правни информации</p>
                    <nav class="flex flex-col gap-3">
                        <Link
                            v-for="link in legalLinks"
                            :key="link.label"
                            :href="link.href"
                            class="w-fit text-sm text-gray-600 transition-colors hover:text-primary"
                        >
                            {{ link.label }}
                        </Link>
                    </nav>
                </div>
            </div>

            <div class="mt-12 border-t border-gray-200 pt-6 text-xs text-gray-400">
                &copy; {{ new Date().getFullYear() }} Фондација „Адријан Живков". Сите права задржани.
            </div>
        </AppContainer>
    </footer>
</template>
