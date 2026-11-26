<script setup lang="ts">
import { privacy, terms } from '@/actions/App/Http/Controllers/LegalController';
import { store } from '@/actions/App/Http/Controllers/StorySubmissionController';
import AppContainer from '@/components/base/containers/AppContainer.vue';
import AppSection from '@/components/base/sections/AppSection.vue';
import PageHero from '@/components/base/sections/PageHero.vue';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import { Head, Link, useForm, usePage } from '@inertiajs/vue3';
import { computed } from 'vue';

defineOptions({
    layout: DefaultLayout,
});

const page = usePage();
const successMessage = computed(() => page.props.flash?.success as string | undefined);

const form = useForm({
    name: '',
    phone: '',
    message: '',
    agreed: false,
});

const inputClass =
    'w-full rounded-xl border border-gray-200 bg-white px-4 py-3.5 text-base text-gray-900 transition-colors duration-200 outline-none placeholder:text-gray-400 focus:border-primary focus:ring-2 focus:ring-primary/15 sm:text-sm';

function submit(): void {
    form.post(store.url(), {
        onSuccess: () => form.reset(),
    });
}
</script>

<template>
    <Head title="Сподели ја својата приказна" />

    <PageHero
        eyebrow="Твојата приказна"
        title="Сподели ја својата приказна"
        description="Доколку поминуваш низ тешкотии и имаш потреба од помош, сакаме да слушнеме за тебе. Пополни ја формата подолу и ние ќе се обидеме да стапиме во контакт со тебе."
    />

    <AppSection>
        <AppContainer>
            <div class="mx-auto max-w-2xl">
                <div v-reveal class="rounded-2xl border border-gray-200 bg-white p-6 sm:p-8 md:p-10">
                    <!-- Success message -->
                    <Transition
                        enter-active-class="transition duration-500 ease-(--ease-soft)"
                        enter-from-class="opacity-0 -translate-y-2"
                        enter-to-class="opacity-100 translate-y-0"
                    >
                        <div v-if="successMessage" class="mb-8 flex items-start gap-3 rounded-xl bg-primary-light px-5 py-4">
                            <svg class="mt-0.5 h-5 w-5 shrink-0 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m4.5 12.75 6 6 9-13.5" />
                            </svg>
                            <p class="font-medium text-primary">{{ successMessage }}</p>
                        </div>
                    </Transition>

                    <!-- Form -->
                    <form @submit.prevent="submit" class="flex flex-col gap-6">
                        <div class="flex flex-col gap-1.5">
                            <label for="name" class="text-sm font-semibold text-gray-900">Име и презиме <span class="text-red-500">*</span></label>
                            <input
                                id="name"
                                v-model="form.name"
                                type="text"
                                placeholder="Вашето име и презиме"
                                :class="[inputClass, form.errors.name ? 'border-red-400 focus:border-red-400 focus:ring-red-100' : '']"
                            />
                            <p v-if="form.errors.name" class="text-sm text-red-500">{{ form.errors.name }}</p>
                        </div>

                        <div class="flex flex-col gap-1.5">
                            <label for="phone" class="text-sm font-semibold text-gray-900">Телефонски број <span class="text-red-500">*</span></label>
                            <input
                                id="phone"
                                v-model="form.phone"
                                type="tel"
                                placeholder="+389 70 000 000"
                                :class="[inputClass, form.errors.phone ? 'border-red-400 focus:border-red-400 focus:ring-red-100' : '']"
                            />
                            <p v-if="form.errors.phone" class="text-sm text-red-500">{{ form.errors.phone }}</p>
                        </div>

                        <div class="flex flex-col gap-1.5">
                            <label for="message" class="text-sm font-semibold text-gray-900"
                                >Твојата приказна <span class="text-red-500">*</span></label
                            >
                            <textarea
                                id="message"
                                v-model="form.message"
                                rows="7"
                                placeholder="Раскажи ни ја твојата ситуација и каква помош ти е потребна..."
                                :class="[
                                    inputClass,
                                    'resize-none',
                                    form.errors.message ? 'border-red-400 focus:border-red-400 focus:ring-red-100' : '',
                                ]"
                            ></textarea>
                            <p v-if="form.errors.message" class="text-sm text-red-500">{{ form.errors.message }}</p>
                        </div>

                        <!-- Legal agreement -->
                        <div class="flex flex-col gap-1.5">
                            <label class="flex cursor-pointer items-start gap-3">
                                <input
                                    v-model="form.agreed"
                                    type="checkbox"
                                    class="mt-0.5 h-4 w-4 shrink-0 cursor-pointer rounded border-gray-300 text-primary focus:ring-primary/30"
                                    :class="{ 'border-red-400': form.errors.agreed }"
                                />
                                <span class="text-sm leading-relaxed text-gray-600">
                                    Се согласувам со
                                    <Link
                                        :href="terms.url()"
                                        target="_blank"
                                        class="font-medium text-primary underline underline-offset-2 hover:text-primary-dark"
                                        >Условите на користење</Link
                                    >
                                    и
                                    <Link
                                        :href="privacy.url()"
                                        target="_blank"
                                        class="font-medium text-primary underline underline-offset-2 hover:text-primary-dark"
                                        >Политиката за приватност</Link
                                    >
                                    на Фондацијата „Адријан Живков".
                                </span>
                            </label>
                            <p v-if="form.errors.agreed" class="text-sm text-red-500">{{ form.errors.agreed }}</p>
                        </div>

                        <button
                            type="submit"
                            :disabled="form.processing"
                            class="group inline-flex w-full items-center justify-center gap-2 rounded-full bg-primary px-8 py-4 text-sm font-semibold text-white transition-colors duration-300 hover:bg-primary-dark disabled:cursor-not-allowed disabled:opacity-60 sm:w-auto"
                        >
                            <svg v-if="form.processing" class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                            </svg>
                            {{ form.processing ? 'Се испраќа...' : 'Испрати ја приказната' }}
                        </button>
                    </form>
                </div>
            </div>
        </AppContainer>
    </AppSection>
</template>
