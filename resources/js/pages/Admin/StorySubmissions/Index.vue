<script setup lang="ts">
import { edit, index } from '@/actions/App/Http/Controllers/Admin/StorySubmissionController';
import PageHeader from '@/components/admin/PageHeader.vue';
import Pagination from '@/components/admin/Pagination.vue';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useAdminFilters } from '@/composables/useAdminFilters';
import type { PaginatedInterface } from '@/interfaces/paginated-interface';
import type { StorySubmissionInterface, StorySubmissionStatusOptionInterface } from '@/interfaces/story-submission-interface';
import { formatDateTime } from '@/lib/formatters';
import { Link, router } from '@inertiajs/vue3';
import { Inbox, Search } from '@lucide/vue';

const props = defineProps<{
    storySubmissions: PaginatedInterface<StorySubmissionInterface>;
    statuses: StorySubmissionStatusOptionInterface[];
    counts: Record<string, number>;
    filters: { search: string; status: string };
}>();

const filters = useAdminFilters(index().url, { ...props.filters });

function statusOption(value: string): StorySubmissionStatusOptionInterface | undefined {
    return props.statuses.find((status) => status.value === value);
}
</script>

<template>
    <div class="space-y-6">
        <PageHeader title="Поднесени приказни" description="Приказните испратени преку формата „Сподели ја својата приказна“ на сајтот." />

        <Card class="gap-0 py-0">
            <CardContent class="flex flex-col gap-3 border-b p-4 lg:flex-row lg:items-center">
                <Tabs :model-value="filters.status || 'all'" @update:model-value="(value) => (filters.status = value === 'all' ? '' : String(value))">
                    <TabsList class="h-auto max-w-full flex-wrap justify-start">
                        <TabsTrigger value="all">
                            Сите
                            <Badge variant="neutral">{{ counts.all ?? 0 }}</Badge>
                        </TabsTrigger>
                        <TabsTrigger v-for="status in statuses" :key="status.value" :value="status.value">
                            {{ status.label }}
                            <Badge variant="neutral">{{ counts[status.value] ?? 0 }}</Badge>
                        </TabsTrigger>
                    </TabsList>
                </Tabs>

                <div class="relative sm:w-72 lg:ml-auto">
                    <Search class="text-muted-foreground absolute top-1/2 left-2.5 size-4 -translate-y-1/2" />
                    <Input v-model="filters.search" type="search" placeholder="Име" class="pl-8" aria-label="Пребарај" />
                </div>
            </CardContent>

            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead class="pl-4">Име</TableHead>
                        <TableHead>Телефон</TableHead>
                        <TableHead>Приказна</TableHead>
                        <TableHead>Статус</TableHead>
                        <TableHead class="pr-4">Примено</TableHead>
                    </TableRow>
                </TableHeader>

                <TableBody>
                    <TableRow
                        v-for="submission in storySubmissions.data"
                        :key="submission.id"
                        class="cursor-pointer"
                        @click="router.visit(edit(submission.id).url)"
                    >
                        <TableCell class="pl-4">
                            <Link :href="edit(submission.id).url" class="font-medium whitespace-nowrap hover:underline" @click.stop>{{
                                submission.name
                            }}</Link>
                        </TableCell>
                        <TableCell class="whitespace-nowrap tabular-nums">{{ submission.phone }}</TableCell>
                        <TableCell class="text-muted-foreground max-w-md">
                            <p class="truncate" :title="submission.message">{{ submission.message }}</p>
                        </TableCell>
                        <TableCell>
                            <Badge :variant="statusOption(submission.status)?.color ?? 'neutral'">{{
                                statusOption(submission.status)?.label ?? submission.status
                            }}</Badge>
                        </TableCell>
                        <TableCell class="text-muted-foreground pr-4 whitespace-nowrap tabular-nums">{{
                            formatDateTime(submission.created_at)
                        }}</TableCell>
                    </TableRow>

                    <TableRow v-if="storySubmissions.data.length === 0">
                        <TableCell colspan="5" class="h-40 text-center">
                            <div class="text-muted-foreground flex flex-col items-center gap-2">
                                <Inbox class="size-6" />
                                Нема приказни за овие филтри.
                            </div>
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>

            <CardContent class="border-t p-4">
                <Pagination :paginator="storySubmissions" />
            </CardContent>
        </Card>
    </div>
</template>
