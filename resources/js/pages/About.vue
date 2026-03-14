<script setup lang="ts">
import { create as storyCreate } from '@/actions/App/Http/Controllers/StorySubmissionController';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import { Head, Link } from '@inertiajs/vue3';

defineOptions({
    layout: DefaultLayout,
});
</script>

<template>
    <Head title="За фондацијата" />

    <!-- Hero -->
    <div class="relative m-3 overflow-hidden rounded-2xl bg-[#054A29]">
        <div class="pointer-events-none absolute top-0 right-0 h-96 w-96 translate-x-1/3 -translate-y-1/3 rounded-full bg-white/5"></div>
        <div class="pointer-events-none absolute bottom-0 left-0 h-64 w-64 -translate-x-1/3 translate-y-1/3 rounded-full bg-white/5"></div>

        <AppContainer class="relative py-14 md:py-20">
            <div class="grid grid-cols-1 items-center gap-10 lg:grid-cols-2">
                <div class="flex flex-col gap-6">
                    <div class="inline-flex w-fit items-center gap-2 rounded-full border border-white/20 bg-white/10 px-3 py-1.5">
                        <div class="h-1.5 w-1.5 rounded-full bg-emerald-300"></div>
                        <span class="text-xs font-semibold tracking-widest text-white/80 uppercase">За фондацијата</span>
                    </div>

                    <h1 class="text-4xl leading-tight font-bold tracking-tight text-white md:text-5xl lg:text-[52px] lg:leading-[1.1]">
                        Фондација<br />„Адријан Живков"
                    </h1>

                    <blockquote class="border-l-2 border-white/30 py-1 pl-5">
                        <p class="text-base leading-relaxed text-white/80 italic lg:text-lg">
                            „Во тие кобни моменти, Адријан херојски и без двоумење го жртвуваше сопствениот живот, обидувајќи се да спаси што е можно
                            поголем број луѓе."
                        </p>
                    </blockquote>

                    <p class="max-w-lg text-base leading-relaxed text-white/70">
                        Основана во чест на Адријан Живков, млад херој кој го даде животот за другите. Нашата мисија е да ја продолжиме неговата
                        приказна преку дела на солидарност и помош.
                    </p>
                </div>

                <div class="flex justify-center lg:justify-end">
                    <div class="relative">
                        <div class="absolute inset-0 rounded-2xl bg-white/10 blur-2xl"></div>
                        <img
                            src="/images/pages/home/who-is-adrijan-section/adrijan-desktop.webp"
                            alt="Адријан Живков"
                            class="relative hidden w-full max-w-sm rounded-2xl shadow-2xl lg:block"
                        />
                        <img
                            src="/images/pages/home/who-is-adrijan-section/adrijan-mobile.webp"
                            alt="Адријан Живков"
                            class="relative w-full max-w-sm rounded-2xl shadow-2xl lg:hidden"
                        />
                    </div>
                </div>
            </div>
        </AppContainer>
    </div>

    <!-- His story -->
    <div class="py-16 md:py-20">
        <AppContainer>
            <div class="mx-auto max-w-3xl">
                <p class="mb-2 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Неговата приказна</p>
                <h2 class="mb-8 text-3xl font-bold tracking-tight text-gray-900 md:text-4xl">Кој беше Адријан?</h2>

                <div class="space-y-5 text-base leading-relaxed text-gray-600">
                    <p>
                        Адријан Живков беше млад човек со исклучително голема душа. Пријател, син и брат кој воодушевуваше со својата топлина,
                        насмевка и несебичност. Животот го живееше со грижа за луѓето околу него.
                    </p>
                    <p>
                        На 16 март 2025 година, во дискотеката „Пулс" во Кочани, настана трагедија која засекогаш ја промени Македонија. Во настанот
                        загинаа 63 лица, а уште 193 беа повредени.
                    </p>
                    <p>
                        Во тие хаотични моменти, Адријан покажа каков човек навистина беше. Наместо да побегне, се врати по своите пријатели, помагаше
                        на непознати и го жртвуваше сопствениот живот за другите. Неговото дело останува.
                    </p>
                </div>
            </div>
        </AppContainer>
    </div>

    <!-- Divider -->
    <div class="border-t border-gray-100"></div>

    <!-- Why the foundation -->
    <div class="py-16 md:py-20">
        <AppContainer>
            <div class="grid grid-cols-1 gap-12 lg:grid-cols-2 lg:items-center">
                <div>
                    <p class="mb-2 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Зошто постоиме</p>
                    <h2 class="mb-6 text-3xl font-bold tracking-tight text-gray-900 md:text-4xl">Зошто ја основавме фондацијата?</h2>
                    <div class="space-y-4 text-base leading-relaxed text-gray-600">
                        <p>
                            Фондацијата „Адријан Живков" ја основавме неговите пријатели и семејство, луѓе кои го познаваа, го сакаа и сè уште го
                            носат во срцето. Не сакавме болката да остане само болка. Сакавме да ја претвориме во нешто трајно: во помош, во надеж и
                            во светлина за оние кои имаат потреба.
                        </p>
                        <p>
                            Адријан живееше за другите. Најмалку што можевме да направиме е да го продолжиме тоа. Секој пат кога помагаме на некого,
                            тој е присутен.
                        </p>
                    </div>
                </div>

                <div class="flex flex-col gap-4">
                    <div class="rounded-2xl border border-gray-100 bg-gray-50 p-6">
                        <p class="mb-1 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Основана</p>
                        <p class="text-lg font-semibold text-gray-900">2025 година</p>
                        <p class="mt-1 text-sm text-gray-500">Во чест на Адријан Живков, по трагедијата во Кочани.</p>
                    </div>
                    <div class="rounded-2xl border border-gray-100 bg-gray-50 p-6">
                        <p class="mb-1 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Основачи</p>
                        <p class="text-lg font-semibold text-gray-900">Пријатели и семејство</p>
                        <p class="mt-1 text-sm text-gray-500">Луѓе кои го познаваа и сакаа Адријан лично.</p>
                    </div>
                    <div class="rounded-2xl border border-gray-100 bg-gray-50 p-6">
                        <p class="mb-1 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Мисија</p>
                        <p class="text-lg font-semibold text-gray-900">Помош на луѓе во потреба</p>
                        <p class="mt-1 text-sm text-gray-500">
                            Директна поддршка за поединци и семејства кои поминуваат низ тешки животни околности.
                        </p>
                    </div>
                </div>
            </div>
        </AppContainer>
    </div>

    <!-- Values -->
    <div class="bg-gray-50 py-16 md:py-20">
        <AppContainer>
            <div class="mb-12 text-center">
                <p class="mb-2 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Нашите вредности</p>
                <h2 class="text-3xl font-bold tracking-tight text-gray-900 md:text-4xl">Вредностите кои го водеа Адријан</h2>
                <p class="mx-auto mt-4 max-w-xl text-base text-gray-500">
                    Овие вредности не се само зборови. Тие беа начинот на кој Адријан живееше секој ден.
                </p>
            </div>

            <div class="grid grid-cols-1 gap-6 md:grid-cols-3">
                <div class="flex flex-col gap-4 rounded-2xl border border-gray-200 bg-white p-8 shadow-sm">
                    <div class="flex h-12 w-12 items-center justify-center rounded-2xl bg-[#054A29]/10">
                        <svg class="h-6 w-6 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="1.75"
                                d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"
                            />
                        </svg>
                    </div>
                    <div>
                        <h3 class="mb-2 text-xl font-bold text-gray-900">Солидарност</h3>
                        <p class="text-sm leading-relaxed text-gray-500">
                            Да стоиш рамо до рамо со некого во неговиот најтежок момент. Не зашто мораш, туку зашто сакаш.
                        </p>
                    </div>
                </div>

                <div class="flex flex-col gap-4 rounded-2xl border border-gray-200 bg-white p-8 shadow-sm">
                    <div class="flex h-12 w-12 items-center justify-center rounded-2xl bg-[#054A29]/10">
                        <svg class="h-6 w-6 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="1.75"
                                d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                            />
                        </svg>
                    </div>
                    <div>
                        <h3 class="mb-2 text-xl font-bold text-gray-900">Емпатија</h3>
                        <p class="text-sm leading-relaxed text-gray-500">
                            Да ја почувствуваш болката на другиот како своја. Да гледаш човек таму каде другите гледаат само проблем.
                        </p>
                    </div>
                </div>

                <div class="flex flex-col gap-4 rounded-2xl border border-gray-200 bg-white p-8 shadow-sm">
                    <div class="flex h-12 w-12 items-center justify-center rounded-2xl bg-[#054A29]/10">
                        <svg class="h-6 w-6 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="1.75"
                                d="M12 8v13m0-13V6a2 2 0 112 2h-2zm0 0V5.5A2.5 2.5 0 109.5 8H12zm-7 4h14M5 12a2 2 0 110-4h14a2 2 0 110 4M5 12v7a2 2 0 002 2h10a2 2 0 002-2v-7"
                            />
                        </svg>
                    </div>
                    <div>
                        <h3 class="mb-2 text-xl font-bold text-gray-900">Несебичност</h3>
                        <p class="text-sm leading-relaxed text-gray-500">
                            Да дадеш без да очекуваш нешто назад. Да ставиш некој друг пред себе, дури и кога е најтешко.
                        </p>
                    </div>
                </div>
            </div>
        </AppContainer>
    </div>

    <!-- Transparency -->
    <div class="py-16 md:py-20">
        <AppContainer>
            <div class="mx-auto max-w-3xl">
                <p class="mb-2 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Транспарентност</p>
                <h2 class="mb-8 text-3xl font-bold tracking-tight text-gray-900 md:text-4xl">Како работиме</h2>

                <div class="space-y-4">
                    <div class="flex gap-5 rounded-2xl border border-gray-100 p-6">
                        <div class="mt-0.5 flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-[#054A29]/10">
                            <svg class="h-5 w-5 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="1.75"
                                    d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                />
                            </svg>
                        </div>
                        <div>
                            <h3 class="mb-1 font-semibold text-gray-900">Финансирани од пријатели и семејство</h3>
                            <p class="text-sm leading-relaxed text-gray-500">
                                Фондацијата е финансирана исклучиво преку доброволни донации од пријателите и семејството на Адријан. Не прифаќаме
                                донации од трети лица.
                            </p>
                        </div>
                    </div>

                    <div class="flex gap-5 rounded-2xl border border-gray-100 p-6">
                        <div class="mt-0.5 flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-[#054A29]/10">
                            <svg class="h-5 w-5 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="1.75"
                                    d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z"
                                />
                            </svg>
                        </div>
                        <div>
                            <h3 class="mb-1 font-semibold text-gray-900">Донациите одат директно до луѓето</h3>
                            <p class="text-sm leading-relaxed text-gray-500">
                                Фондацијата не собира, не прима и не посредува со пари. Секоја донација оди директно од донаторот до лицето во
                                потреба, без посредник.
                            </p>
                        </div>
                    </div>

                    <div class="flex gap-5 rounded-2xl border border-gray-100 p-6">
                        <div class="mt-0.5 flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-[#054A29]/10">
                            <svg class="h-5 w-5 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.75" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="1.75"
                                    d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                                />
                            </svg>
                        </div>
                        <div>
                            <h3 class="mb-1 font-semibold text-gray-900">Нашата улога е видливост</h3>
                            <p class="text-sm leading-relaxed text-gray-500">
                                Ги презентираме приказните на луѓето кои имаат потреба, ги верификуваме колку е можно и ги поврзуваме со добронамерни
                                луѓе кои сакаат да помогнат.
                            </p>
                        </div>
                    </div>

                    <div class="flex gap-5 rounded-2xl border border-gray-100 p-6">
                        <div class="mt-0.5 flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-[#054A29]/10">
                            <svg class="h-5 w-5 text-[#054A29]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="1.75"
                                    d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                                />
                            </svg>
                        </div>
                        <div>
                            <h3 class="mb-1 font-semibold text-gray-900">Доброволно и со почит</h3>
                            <p class="text-sm leading-relaxed text-gray-500">
                                Секоја приказна се објавува со согласност на засегнатото лице. Никогаш не вршиме притисок, ниту кон оние кои бараат
                                помош, ниту кон оние кои сакаат да дадат.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </AppContainer>
    </div>

    <!-- CTA -->
    <div class="m-3 rounded-2xl bg-[#054A29] py-14 md:py-16">
        <AppContainer class="text-center text-white">
            <h2 class="mb-4 text-3xl font-bold tracking-tight md:text-4xl">Имаш потреба од помош?</h2>
            <p class="mx-auto mb-8 max-w-lg text-base text-white/70 md:text-lg">
                Доколку поминуваш низ тежок период, сподели ја својата приказна. Ќе направиме сè што е во наша моќ да помогнеме.
            </p>
            <Link
                :href="storyCreate.url()"
                class="inline-flex items-center rounded-xl bg-white px-8 py-3.5 text-sm font-semibold text-[#054A29] shadow-lg transition-all duration-200 hover:bg-white/90 hover:shadow-xl"
            >
                Побарај помош
            </Link>
        </AppContainer>
    </div>
</template>
