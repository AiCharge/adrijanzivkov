<script setup lang="ts">
import { index, update } from '@/actions/App/Http/Controllers/Admin/NewsController';
import { show } from '@/actions/App/Http/Controllers/NewsController';
import PageHeader from '@/components/admin/PageHeader.vue';
import RecordBackLink from '@/components/admin/RecordBackLink.vue';
import NewsFields, { type NewsFormData } from '@/components/pages/admin/NewsFields.vue';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import type { NewsInterface } from '@/interfaces/news-interface';
import { formatDateTime } from '@/lib/formatters';
import { Link, setLayoutProps, useForm } from '@inertiajs/vue3';
import { ExternalLink, Save } from '@lucide/vue';

const props = defineProps<{
    news: NewsInterface;
}>();

setLayoutProps({ breadcrumb: props.news.title });

const form = useForm<NewsFormData>({
    title: props.news.title,
    short_description: props.news.short_description,
    content: props.news.content ?? '',
    main_image: null,
});

function submit(): void {
    form.transform((data) => ({ ...data, _method: 'put' })).post(update(props.news.slug).url, {
        preserveScroll: true,
        onSuccess: () => form.reset('main_image'),
    });
}
</script>

<template>
    <div class="max-w-4xl space-y-6">
        <RecordBackLink :href="index().url" label="Новости" />

        <PageHeader :title="news.title" :description="`Објавено на ${formatDateTime(news.created_at)}`">
            <template #actions>
                <Button variant="outline" as-child>
                    <a :href="show(news.slug).url" target="_blank" rel="noopener">
                        <ExternalLink />
                        Види на сајтот
                    </a>
                </Button>
            </template>
        </PageHeader>

        <form class="space-y-6" @submit.prevent="submit">
            <Card>
                <CardContent>
                    <NewsFields v-model:form="form" :news="news" />
                </CardContent>
            </Card>

            <div class="flex justify-end gap-2">
                <Button variant="outline" as-child>
                    <Link :href="index().url">Назад</Link>
                </Button>
                <Button type="submit" :disabled="form.processing">
                    <Save />
                    Зачувај
                </Button>
            </div>
        </form>
    </div>
</template>
