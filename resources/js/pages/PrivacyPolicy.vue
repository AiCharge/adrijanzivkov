<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import { Head } from '@inertiajs/vue3';

defineOptions({
    layout: DefaultLayout,
});
</script>

<template>
    <Head title="Политика за приватност" />

    <div class="min-h-screen py-10 md:py-16">
        <AppContainer>
            <div class="mx-auto max-w-3xl">
                <h1 class="mb-2 text-3xl font-bold tracking-tight text-gray-900 md:text-4xl">Политика за приватност</h1>
                <p class="mb-10 text-sm text-gray-400">Последно ажурирање: март 2026</p>

                <div class="space-y-10 text-gray-700">
                    <section>
                        <h2 class="mb-3 text-xl font-semibold text-gray-900">1. Вовед</h2>
                        <p class="leading-relaxed">
                            Фондацијата „Адријан Живков" (во натамошниот текст: „Фондацијата", „ние", „нас") ја почитува приватноста на сите
                            посетители на оваа веб-страница (во натамошниот текст: „Страницата"). Оваа Политика за приватност ја објаснува нашата
                            практика во однос на личните податоци.
                        </p>
                    </section>

                    <section>
                        <h2 class="mb-3 text-xl font-semibold text-gray-900">2. Податоци кои ги собираме</h2>
                        <p class="leading-relaxed">
                            <strong>Фондацијата не собира, не обработува и не складира лични податоци на посетителите на Страницата.</strong>
                            Не постои регистрација, најава, контакт форма, ни каков било друг механизам преку кој би ги добивале вашите лични
                            информации.
                        </p>
                        <p class="mt-3 leading-relaxed">Единствените содржини кои ги складираме на нашите сервери се:</p>
                        <ul class="mt-3 list-disc space-y-2 pl-6 leading-relaxed">
                            <li><strong>Новости</strong> — приказни и вести поврзани со активностите на Фондацијата;</li>
                            <li><strong>Потреби за донации</strong> — огласи за лица кои имаат потреба од помош, објавени со нивна согласност.</li>
                        </ul>
                        <p class="mt-3 leading-relaxed">Овие содржини се јавни и не содржат технички лични податоци на посетителите.</p>
                    </section>

                    <section>
                        <h2 class="mb-3 text-xl font-semibold text-gray-900">3. Колачиња (Cookies)</h2>
                        <p class="leading-relaxed">
                            Страницата може да користи технички неопходни колачиња исклучиво за обезбедување на основното функционирање. Не користиме
                            колачиња за следење, аналитика или маркетинг. Никакви лични податоци не се собираат преку колачиња.
                        </p>
                    </section>

                    <section>
                        <h2 class="mb-3 text-xl font-semibold text-gray-900">4. Донации</h2>
                        <p class="leading-relaxed">
                            Фондацијата не учествува во финансиски трансакции. Донациите се вршат директно меѓу донаторот и лицето во потреба, без
                            никакво посредување од страна на Фондацијата. Со тоа, Фондацијата не собира никакви финансиски ни платежни податоци.
                        </p>
                    </section>

                    <section>
                        <h2 class="mb-3 text-xl font-semibold text-gray-900">5. Надворешни врски</h2>
                        <p class="leading-relaxed">
                            Страницата може да содржи врски до надворешни веб-страници (на пример профили на социјални мрежи). Фондацијата не сноси
                            одговорност за политиките за приватност на тие страници и ве поттикнуваме сами да се запознаете со нив.
                        </p>
                    </section>

                    <section>
                        <h2 class="mb-3 text-xl font-semibold text-gray-900">6. Измени на Политиката</h2>
                        <p class="leading-relaxed">
                            Оваа Политика за приватност може да се ажурира периодично. Датумот на последното ажурирање е наведен на почетокот на
                            документот. Препорачуваме периодично да ја прегледувате.
                        </p>
                    </section>

                    <section>
                        <h2 class="mb-3 text-xl font-semibold text-gray-900">7. Контакт</h2>
                        <p class="leading-relaxed">За прашања поврзани со оваа Политика за приватност, контактирајте не на:</p>
                        <p class="mt-3 leading-relaxed">
                            <strong>Фондација „Адријан Живков"</strong><br />
                            <a
                                href="mailto:adrijanfoundation@gmail.com"
                                class="font-medium text-[#054A29] underline underline-offset-2 hover:text-[#033d22]"
                            >
                                adrijanfoundation@gmail.com
                            </a>
                        </p>
                    </section>
                </div>
            </div>
        </AppContainer>
    </div>
</template>
