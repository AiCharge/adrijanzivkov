<script setup lang="ts">
import AppContainer from '@/components/base/containers/AppContainer.vue';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import { terms, privacy } from '@/actions/App/Http/Controllers/LegalController';
import { store } from '@/actions/App/Http/Controllers/StorySubmissionController';
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

function submit(): void {
    form.post(store.url(), {
        onSuccess: () => form.reset(),
    });
}
</script>

<template>
    <Head title="Сподели ја својата приказна" />

    <div class="min-h-screen py-10 md:py-16">
        <AppContainer>
            <div class="mx-auto max-w-2xl">
                <!-- Header -->
                <div class="mb-10">
                    <p class="mb-2 text-xs font-semibold tracking-widest text-[#054A29] uppercase">Твојата приказна</p>
                    <h1 class="mb-4 text-3xl font-bold tracking-tight text-gray-900 md:text-4xl">Сподели ја својата приказна</h1>
                    <p class="text-base leading-relaxed text-gray-500 md:text-lg">
                        Доколку поминуваш низ тешкотии и имаш потреба од помош, сакаме да слушнеме за тебе. Пополни ја формата подолу и ние ќе се
                        обидеме да стапиме во контакт со тебе.
                    </p>
                </div>

                <!-- Success message -->
                <div v-if="successMessage" class="mb-8 rounded-2xl bg-[#054A29]/10 px-6 py-5">
                    <p class="font-medium text-[#054A29]">{{ successMessage }}</p>
                </div>

                <!-- Form -->
                <form @submit.prevent="submit" class="flex flex-col gap-6">
                    <div class="flex flex-col gap-1.5">
                        <label for="name" class="text-sm font-medium text-gray-700">Име и презиме <span class="text-red-500">*</span></label>
                        <input
                            id="name"
                            v-model="form.name"
                            type="text"
                            placeholder="Вашето име и презиме"
                            class="w-full rounded-xl border border-gray-200 px-4 py-3 text-sm text-gray-900 transition-colors outline-none placeholder:text-gray-400 focus:border-[#054A29] focus:ring-2 focus:ring-[#054A29]/20"
                            :class="{ 'border-red-400 focus:border-red-400 focus:ring-red-100': form.errors.name }"
                        />
                        <p v-if="form.errors.name" class="text-sm text-red-500">{{ form.errors.name }}</p>
                    </div>

                    <div class="flex flex-col gap-1.5">
                        <label for="phone" class="text-sm font-medium text-gray-700">Телефонски број <span class="text-red-500">*</span></label>
                        <input
                            id="phone"
                            v-model="form.phone"
                            type="tel"
                            placeholder="+389 70 000 000"
                            class="w-full rounded-xl border border-gray-200 px-4 py-3 text-sm text-gray-900 transition-colors outline-none placeholder:text-gray-400 focus:border-[#054A29] focus:ring-2 focus:ring-[#054A29]/20"
                            :class="{ 'border-red-400 focus:border-red-400 focus:ring-red-100': form.errors.phone }"
                        />
                        <p v-if="form.errors.phone" class="text-sm text-red-500">{{ form.errors.phone }}</p>
                    </div>

                    <div class="flex flex-col gap-1.5">
                        <label for="message" class="text-sm font-medium text-gray-700">Твојата приказна <span class="text-red-500">*</span></label>
                        <textarea
                            id="message"
                            v-model="form.message"
                            rows="7"
                            placeholder="Раскажи ни ја твојата ситуација и каква помош ти е потребна..."
                            class="w-full resize-none rounded-xl border border-gray-200 px-4 py-3 text-sm text-gray-900 transition-colors outline-none placeholder:text-gray-400 focus:border-[#054A29] focus:ring-2 focus:ring-[#054A29]/20"
                            :class="{ 'border-red-400 focus:border-red-400 focus:ring-red-100': form.errors.message }"
                        ></textarea>
                        <p v-if="form.errors.message" class="text-sm text-red-500">{{ form.errors.message }}</p>
                    </div>

                    <!-- Legal agreement -->
                    <div class="flex flex-col gap-1.5">
                        <label class="flex cursor-pointer items-start gap-3">
                            <input
                                v-model="form.agreed"
                                type="checkbox"
                                class="mt-0.5 h-4 w-4 shrink-0 cursor-pointer rounded border-gray-300 text-[#054A29] focus:ring-[#054A29]/30"
                                :class="{ 'border-red-400': form.errors.agreed }"
                            />
                            <span class="text-sm leading-relaxed text-gray-600">
                                Се согласувам со
                                <Link
                                    :href="terms.url()"
                                    target="_blank"
                                    class="font-medium text-[#054A29] underline underline-offset-2 hover:text-[#033d22]"
                                    >Условите на користење</Link
                                >
                                и
                                <Link
                                    :href="privacy.url()"
                                    target="_blank"
                                    class="font-medium text-[#054A29] underline underline-offset-2 hover:text-[#033d22]"
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
                        class="inline-flex w-full items-center justify-center gap-2 rounded-xl bg-[#054A29] px-6 py-3.5 text-sm font-semibold text-white transition-colors hover:bg-[#033d22] disabled:cursor-not-allowed disabled:opacity-60 sm:w-auto"
                    >
                        <svg v-if="form.processing" class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                        </svg>
                        {{ form.processing ? 'Се испраќа...' : 'Испрати ја приказната' }}
                    </button>
                </form>
            </div>
        </AppContainer>
    </div>
</template>
