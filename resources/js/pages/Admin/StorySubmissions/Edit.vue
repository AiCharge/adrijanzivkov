<script setup lang="ts">
import { index, update } from '@/actions/App/Http/Controllers/Admin/StorySubmissionController';
import PageHeader from '@/components/admin/PageHeader.vue';
import RecordBackLink from '@/components/admin/RecordBackLink.vue';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Field, FieldError, FieldGroup, FieldLabel } from '@/components/ui/field';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Textarea } from '@/components/ui/textarea';
import type { StorySubmissionInterface, StorySubmissionStatusOptionInterface } from '@/interfaces/story-submission-interface';
import { formatDateTime } from '@/lib/formatters';
import { Link, setLayoutProps, useForm } from '@inertiajs/vue3';
import { Phone, Save } from '@lucide/vue';
import { computed } from 'vue';

const props = defineProps<{
    storySubmission: StorySubmissionInterface;
    statuses: StorySubmissionStatusOptionInterface[];
}>();

setLayoutProps({ breadcrumb: props.storySubmission.name });

const form = useForm({
    status: props.storySubmission.status,
    notes: props.storySubmission.notes ?? '',
});

const savedStatus = computed(() => props.statuses.find((status) => status.value === props.storySubmission.status));

function submit(): void {
    form.put(update(props.storySubmission.id).url, { preserveScroll: true });
}
</script>

<template>
    <div class="space-y-6">
        <RecordBackLink :href="index().url" label="Поднесени приказни" />

        <PageHeader :title="storySubmission.name" :description="`Примено на ${formatDateTime(storySubmission.created_at)}`">
            <template #actions>
                <Badge v-if="savedStatus" :variant="savedStatus.color">{{ savedStatus.label }}</Badge>
            </template>
        </PageHeader>

        <div class="grid items-start gap-6 lg:grid-cols-5">
            <Card class="lg:col-span-3">
                <CardHeader>
                    <CardTitle>Приказна</CardTitle>
                </CardHeader>
                <CardContent class="space-y-6">
                    <dl class="grid gap-4 text-sm sm:grid-cols-2">
                        <div class="space-y-1">
                            <dt class="text-muted-foreground">Име и презиме</dt>
                            <dd class="font-medium">{{ storySubmission.name }}</dd>
                        </div>
                        <div class="space-y-1">
                            <dt class="text-muted-foreground">Телефонски број</dt>
                            <dd>
                                <a
                                    :href="`tel:${storySubmission.phone}`"
                                    class="inline-flex items-center gap-1.5 font-medium text-primary hover:underline"
                                >
                                    <Phone class="size-3.5" />
                                    {{ storySubmission.phone }}
                                </a>
                            </dd>
                        </div>
                    </dl>
                    <p class="text-sm leading-relaxed whitespace-pre-line">{{ storySubmission.message }}</p>
                </CardContent>
            </Card>

            <form class="space-y-6 lg:col-span-2" @submit.prevent="submit">
                <Card>
                    <CardHeader>
                        <CardTitle>Обработка</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <FieldGroup>
                            <Field :data-invalid="!!form.errors.status">
                                <FieldLabel for="status">Статус</FieldLabel>
                                <Select v-model="form.status">
                                    <SelectTrigger id="status" class="w-full" :aria-invalid="!!form.errors.status">
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem v-for="status in statuses" :key="status.value" :value="status.value">{{
                                            status.label
                                        }}</SelectItem>
                                    </SelectContent>
                                </Select>
                                <FieldError :errors="[form.errors.status]" />
                            </Field>
                            <Field :data-invalid="!!form.errors.notes">
                                <FieldLabel for="notes">Белешки</FieldLabel>
                                <Textarea id="notes" v-model="form.notes" rows="6" :aria-invalid="!!form.errors.notes" />
                                <FieldError :errors="[form.errors.notes]" />
                            </Field>
                        </FieldGroup>
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
    </div>
</template>
