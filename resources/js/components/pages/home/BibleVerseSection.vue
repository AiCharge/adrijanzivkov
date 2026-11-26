<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import { computed } from 'vue';

const verses = [
    {
        reference: 'Јован 15:13',
        text: 'Никој нема поголема љубов од оваа: да го положи сопствениот живот за своите пријатели.',
    },
    {
        reference: 'Матеј 25:40',
        text: 'Вистина ви велам, доколку му направивте на еден од овие Мои најмали браќа, Мене Ми направивте.',
    },
    {
        reference: 'Галатјани 6:2',
        text: 'Носете ги бремињата еден на друг, и со тоа ќе го исполните законот Христов.',
    },
    {
        reference: '1 Коринтјани 13:13',
        text: 'А сега остануваат овие три: верата, надежта и љубовта, но љубовта е најголема од нив.',
    },
    {
        reference: 'Римјаните 8:38-39',
        text: 'Зашто сум уверен дека ни смртта, ни животот, ни ангелите, ни властите, ни сегашноста, ни иднината, ни силите, ни височината, ни длабочината, ниту некое друго создание, ќе може да нѐ оддели од Божјата љубов, која е во Христа Исуса, нашиот Господ.',
    },
    {
        reference: 'Псалм 23:4',
        text: 'Ако тргнам и низ долината на смртната сенка, нема да се уплашам од злото, зашто Ти си со мене. Твојот жезал и Твојата палка ме утешуваат.',
    },
    {
        reference: 'Исаија 41:10',
        text: 'Не бој се, зашто Јас Сум со тебе; не обрнувај се исплашено, зашто Јас Сум твој Бог. Јас те крепам и ти помагам, те потпирам со победничка десница.',
    },
    {
        reference: 'Марко 10:45',
        text: 'Зашто и Човечкиот Син не дојде да Му служат, туку да служи и да ја даде Својата душа за откуп на мнозина.',
    },
    {
        reference: 'Притчи 17:17',
        text: 'Пријателот љуби во секое време, а во неволја станува и брат.',
    },
    {
        reference: 'Откровение 21:4',
        text: 'И ќе ја избрише секоја солза од нивните очи, и смрт нема да има веќе, ни тага, ни плач, ниту болка, зашто првото помина.',
    },
];

const verse = computed(() => verses[Math.floor(Math.random() * verses.length)]);
</script>

<template>
    <section class="px-3">
        <div v-reveal class="rounded-3xl bg-primary-light py-16 md:py-24">
            <AppContainer>
                <figure class="mx-auto flex max-w-3xl flex-col items-center gap-8 text-center">
                    <span class="flex h-12 w-12 items-center justify-center rounded-full bg-white text-primary">
                        <svg class="h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="1.75"
                                d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
                            />
                        </svg>
                    </span>
                    <blockquote>
                        <p class="text-xl leading-relaxed font-medium text-balance text-gray-900 italic md:text-2xl lg:text-[28px] lg:leading-snug">
                            „{{ verse.text }}"
                        </p>
                    </blockquote>
                    <figcaption class="text-sm font-semibold tracking-[0.15em] text-primary uppercase">{{ verse.reference }}</figcaption>
                </figure>
            </AppContainer>
        </div>
    </section>
</template>
