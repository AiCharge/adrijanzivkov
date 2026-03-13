<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import LogoDark from '@/components/logos/LogoDark.vue';
import { Link } from '@inertiajs/vue3';
import { route } from 'ziggy-js';
</script>

<template>
    <footer class="border-t border-gray-100 bg-white">
        <AppContainer class="py-10">
            <div class="flex flex-col gap-8 md:flex-row md:items-start md:justify-between">
                <div class="flex flex-col gap-4">
                    <Link :href="route('index')">
                        <LogoDark />
                    </Link>
                    <p class="max-w-xs text-sm leading-relaxed text-gray-500">
                        Фондација посветена на помош, надеж и инспирација за луѓе кои имаат потреба.
                    </p>
                </div>

                <div class="flex flex-col gap-3">
                    <p class="text-xs font-semibold tracking-widest text-gray-400 uppercase">Навигација</p>
                    <nav class="flex flex-col gap-2">
                        <Link :href="route('index') + '#about-adrijan'" class="text-sm text-gray-600 transition-colors hover:text-[#054A29]">
                            За Адријан
                        </Link>
                        <Link :href="route('index') + '#our-mission'" class="text-sm text-gray-600 transition-colors hover:text-[#054A29]">
                            Нашата мисија
                        </Link>
                        <Link :href="route('news.index')" class="text-sm text-gray-600 transition-colors hover:text-[#054A29]"> Новости </Link>
                        <Link :href="route('index') + '#contact'" class="text-sm text-gray-600 transition-colors hover:text-[#054A29]">
                            Контакт
                        </Link>
                    </nav>
                </div>

                <div class="flex flex-col gap-3">
                    <p class="text-xs font-semibold tracking-widest text-gray-400 uppercase">Правни информации</p>
                    <nav class="flex flex-col gap-2">
                        <Link :href="route('terms')" class="text-sm text-gray-600 transition-colors hover:text-[#054A29]"> Услови на користење </Link>
                        <Link :href="route('privacy')" class="text-sm text-gray-600 transition-colors hover:text-[#054A29]">
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
