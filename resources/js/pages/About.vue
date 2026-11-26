<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import SectionHeading from '@/components/base/headings/SectionHeading.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
import HelpCtaSection from '@/components/base/sections/HelpCtaSection.vue';
import PageHero from '@/components/base/sections/PageHero.vue';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import { Head } from '@inertiajs/vue3';

defineOptions({
    layout: DefaultLayout,
});

const facts = [
    { label: 'Основана', value: '2025 година', text: 'Во чест на Адријан Живков, по трагедијата во Кочани.' },
    { label: 'Основачи', value: 'Пријатели и семејство', text: 'Луѓе кои го познаваа и сакаа Адријан лично.' },
    {
        label: 'Мисија',
        value: 'Помош на луѓе во потреба',
        text: 'Директна поддршка за поединци и семејства кои поминуваат низ тешки животни околности.',
    },
];

const values = [
    {
        title: 'Солидарност',
        text: 'Да стоиш рамо до рамо со некого во неговиот најтежок момент. Не зашто мораш, туку зашто сакаш.',
        icon: 'M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z',
    },
    {
        title: 'Емпатија',
        text: 'Да ја почувствуваш болката на другиот како своја. Да гледаш човек таму каде другите гледаат само проблем.',
        icon: 'M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z',
    },
    {
        title: 'Несебичност',
        text: 'Да дадеш без да очекуваш нешто назад. Да ставиш некој друг пред себе, дури и кога е најтешко.',
        icon: 'M12 8v13m0-13V6a2 2 0 112 2h-2zm0 0V5.5A2.5 2.5 0 109.5 8H12zm-7 4h14M5 12a2 2 0 110-4h14a2 2 0 110 4M5 12v7a2 2 0 002 2h10a2 2 0 002-2v-7',
    },
];

const principles = [
    {
        title: 'Финансирани од пријатели и семејство',
        text: 'Фондацијата е финансирана исклучиво преку доброволни донации од пријателите и семејството на Адријан. Не прифаќаме донации од трети лица.',
        icon: 'M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
    },
    {
        title: 'Донациите одат директно до луѓето',
        text: 'Фондацијата не собира, не прима и не посредува со пари. Секоја донација оди директно од донаторот до лицето во потреба, без посредник.',
        icon: 'M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z',
    },
    {
        title: 'Нашата улога е видливост',
        text: 'Ги презентираме приказните на луѓето кои имаат потреба, ги верификуваме колку е можно и ги поврзуваме со добронамерни луѓе кои сакаат да помогнат.',
        icon: 'M15 12a3 3 0 11-6 0 3 3 0 016 0zM2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z',
    },
    {
        title: 'Доброволно и со почит',
        text: 'Секоја приказна се објавува со согласност на засегнатото лице. Никогаш не вршиме притисок, ниту кон оние кои бараат помош, ниту кон оние кои сакаат да дадат.',
        icon: 'M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z',
    },
];
</script>

<template>
    <Head title="За фондацијата" />

    <PageHero eyebrow="За фондацијата" title='Фондација „Адријан Живков"'>
        <div class="flex flex-col gap-6">
            <blockquote class="border-l-2 border-white/30 pl-5">
                <p class="text-base leading-relaxed text-white/80 italic lg:text-lg">
                    „Во тие кобни моменти, Адријан херојски и без двоумење го жртвуваше сопствениот живот, обидувајќи се да спаси што е можно поголем
                    број луѓе."
                </p>
            </blockquote>

            <p class="max-w-lg text-base leading-relaxed text-white/70">
                Основана во чест на Адријан Живков, млад херој кој го даде животот за другите. Нашата мисија е да ја продолжиме неговата приказна
                преку дела на солидарност и помош.
            </p>
        </div>

        <template #aside>
            <div class="flex justify-center lg:justify-end">
                <img
                    src="/images/pages/home/who-is-adrijan-section/adrijan-desktop.webp"
                    alt="Адријан Живков"
                    class="hidden w-full max-w-sm animate-zoom-out rounded-2xl lg:block"
                />
                <img
                    src="/images/pages/home/who-is-adrijan-section/adrijan-mobile.webp"
                    alt="Адријан Живков"
                    class="w-full max-w-sm animate-zoom-out rounded-2xl lg:hidden"
                />
            </div>
        </template>
    </PageHero>

    <AppSection>
        <AppContainer>
            <div class="grid grid-cols-1 gap-10 lg:grid-cols-12 lg:gap-16">
                <div v-reveal class="flex flex-col gap-4 lg:col-span-4">
                    <p class="text-xs font-semibold tracking-[0.2em] text-primary uppercase">Неговата приказна</p>
                    <h2 class="text-3xl leading-[1.1] font-bold tracking-tight text-gray-900 md:text-4xl lg:text-[44px]">Кој беше Адријан?</h2>
                </div>

                <div v-reveal="120" class="flex flex-col gap-5 text-base leading-relaxed text-gray-600 lg:col-span-8 lg:text-lg">
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
    </AppSection>

    <AppSection class="bg-white">
        <AppContainer>
            <div class="grid grid-cols-1 gap-10 lg:grid-cols-12 lg:items-center lg:gap-16">
                <div v-reveal class="flex flex-col gap-6 lg:col-span-6">
                    <p class="text-xs font-semibold tracking-[0.2em] text-primary uppercase">Зошто постоиме</p>
                    <h2 class="text-3xl leading-[1.1] font-bold tracking-tight text-gray-900 md:text-4xl lg:text-[44px]">
                        Зошто ја основавме фондацијата?
                    </h2>
                    <div class="flex flex-col gap-4 text-base leading-relaxed text-gray-600">
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

                <div class="flex flex-col gap-4 lg:col-span-6">
                    <div v-for="(fact, index) in facts" :key="fact.label" v-reveal="index * 100" class="rounded-2xl border border-gray-200 p-6">
                        <p class="mb-1 text-xs font-semibold tracking-[0.2em] text-primary uppercase">{{ fact.label }}</p>
                        <p class="text-lg font-bold text-gray-900">{{ fact.value }}</p>
                        <p class="mt-1 text-sm text-gray-600">{{ fact.text }}</p>
                    </div>
                </div>
            </div>
        </AppContainer>
    </AppSection>

    <AppSection>
        <AppContainer>
            <SectionHeading
                v-reveal
                eyebrow="Нашите вредности"
                title="Вредностите кои го водеа Адријан"
                description="Овие вредности не се само зборови. Тие беа начинот на кој Адријан живееше секој ден."
                centered
                class="mb-12"
            />

            <div class="grid grid-cols-1 gap-6 md:grid-cols-3">
                <div
                    v-for="(value, index) in values"
                    :key="value.title"
                    v-reveal="index * 100"
                    class="flex flex-col gap-6 rounded-2xl border border-gray-200 bg-white p-8"
                >
                    <span class="flex h-12 w-12 items-center justify-center rounded-xl bg-primary-light text-primary">
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.75" :d="value.icon" />
                        </svg>
                    </span>
                    <div class="flex flex-col gap-2">
                        <h3 class="text-xl font-bold text-gray-900">{{ value.title }}</h3>
                        <p class="text-sm leading-relaxed text-gray-600">{{ value.text }}</p>
                    </div>
                </div>
            </div>
        </AppContainer>
    </AppSection>

    <AppSection class="bg-white">
        <AppContainer>
            <div class="grid grid-cols-1 gap-10 lg:grid-cols-12 lg:gap-16">
                <div v-reveal class="flex flex-col gap-4 lg:col-span-4">
                    <p class="text-xs font-semibold tracking-[0.2em] text-primary uppercase">Транспарентност</p>
                    <h2 class="text-3xl leading-[1.1] font-bold tracking-tight text-gray-900 md:text-4xl lg:text-[44px]">Како работиме</h2>
                </div>

                <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:col-span-8">
                    <div
                        v-for="(principle, index) in principles"
                        :key="principle.title"
                        v-reveal="(index % 2) * 100"
                        class="flex flex-col gap-5 rounded-2xl border border-gray-200 p-6"
                    >
                        <span class="flex h-11 w-11 items-center justify-center rounded-xl bg-primary-light text-primary">
                            <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.75" :d="principle.icon" />
                            </svg>
                        </span>
                        <div class="flex flex-col gap-1.5">
                            <h3 class="font-semibold text-gray-900">{{ principle.title }}</h3>
                            <p class="text-sm leading-relaxed text-gray-600">{{ principle.text }}</p>
                        </div>
                    </div>
                </div>
            </div>
        </AppContainer>
    </AppSection>

    <HelpCtaSection />
</template>
