<script setup lang="ts">
import HomeController from '@/actions/App/Http/Controllers/HomeController';
import { privacy, terms } from '@/actions/App/Http/Controllers/LegalController';
import { index as newsIndex } from '@/actions/App/Http/Controllers/NewsController';
import { create as storyCreate } from '@/actions/App/Http/Controllers/StorySubmissionController';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import LogoDark from '@/components/logos/LogoDark.vue';
import { Link } from '@inertiajs/vue3';
</script>

<template>
    <footer class="border-t border-gray-100 bg-white">
        <AppContainer class="py-10">
            <div class="flex flex-col gap-8 md:flex-row md:items-start md:justify-between">
                <div class="flex flex-col gap-4">
                    <Link :href="HomeController.url()">
                        <LogoDark />
                    </Link>
                    <p class="max-w-xs text-sm leading-relaxed text-gray-500">
                        Фондација посветена на помош, надеж и инспирација за луѓе кои имаат потреба.
                    </p>
                </div>

                <div class="flex flex-col gap-3">
                    <p class="text-xs font-semibold tracking-widest text-gray-400 uppercase">Навигација</p>
                    <nav class="flex flex-col gap-2">
                        <Link :href="HomeController.url() + '#about-adrijan'" class="text-sm text-gray-600 transition-colors hover:text-[#054A29]">
                            За Адријан
                        </Link>
                        <Link :href="HomeController.url() + '#our-mission'" class="text-sm text-gray-600 transition-colors hover:text-[#054A29]">
                            Нашата мисија
                        </Link>
                        <Link :href="newsIndex.url()" class="text-sm text-gray-600 transition-colors hover:text-[#054A29]"> Новости </Link>
                        <Link :href="HomeController.url() + '#contact'" class="text-sm text-gray-600 transition-colors hover:text-[#054A29]">
                            Контакт
                        </Link>
                        <Link :href="storyCreate.url()" class="text-sm font-semibold text-[#054A29] transition-colors hover:text-[#033d22]">
                            Побарај помош
                        </Link>
                    </nav>
                </div>

                <div class="flex flex-col gap-3">
                    <p class="text-xs font-semibold tracking-widest text-gray-400 uppercase">Правни информации</p>
                    <nav class="flex flex-col gap-2">
                        <Link :href="terms.url()" class="text-sm text-gray-600 transition-colors hover:text-[#054A29]"> Услови на користење </Link>
                        <Link :href="privacy.url()" class="text-sm text-gray-600 transition-colors hover:text-[#054A29]">
                            Политика за приватност
                        </Link>
                    </nav>
                </div>
            </div>

            <div class="mt-10 border-t border-gray-100 pt-6 text-center text-xs text-gray-400">
                &copy; {{ new Date().getFullYear() }} Фондација „Адријан Живков". Сите права задржани.
            </div>
        </AppContainer>
    </footer>
</template>
