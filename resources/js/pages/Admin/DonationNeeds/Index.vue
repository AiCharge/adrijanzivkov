<script setup lang="ts">
import { create, edit, index } from '@/actions/App/Http/Controllers/Admin/DonationNeedController';
import PageHeader from '@/components/admin/PageHeader.vue';
import Pagination from '@/components/admin/Pagination.vue';
import DonationNeedArchiveButton from '@/components/pages/admin/DonationNeedArchiveButton.vue';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useAdminFilters } from '@/composables/useAdminFilters';
import type { DonationNeedInterface } from '@/interfaces/donation-need-interface';
import type { PaginatedInterface } from '@/interfaces/paginated-interface';
import { formatDateTime, formatMoney } from '@/lib/formatters';
import { Link, router } from '@inertiajs/vue3';
import { HeartHandshake, Plus, Search } from '@lucide/vue';

const props = defineProps<{
    donationNeeds: PaginatedInterface<DonationNeedInterface>;
    counts: { all: number; active: number; archived: number };
    filters: { search: string; archived: string };
}>();

const filters = useAdminFilters(index().url, { ...props.filters });
</script>

<template>
    <div class="space-y-6">
        <PageHeader title="Донации" description="Потребите за донации на сајтот. Потреба што повеќе не е актуелна се архивира.">
            <template #actions>
                <Button as-child>
                    <Link :href="create().url">
                        <Plus />
                        Додај потреба
                    </Link>
                </Button>
            </template>
        </PageHeader>

        <Card class="gap-0 py-0">
            <CardContent class="flex flex-col gap-3 border-b p-4 lg:flex-row lg:items-center">
                <Tabs
                    :model-value="filters.archived || 'all'"
                    @update:model-value="(value) => (filters.archived = value === 'all' ? '' : String(value))"
                >
                    <TabsList class="h-auto max-w-full flex-wrap justify-start">
                        <TabsTrigger value="all">
                            Сите
                            <Badge variant="neutral">{{ counts.all }}</Badge>
                        </TabsTrigger>
                        <TabsTrigger value="active">
                            Активни
                            <Badge variant="neutral">{{ counts.active }}</Badge>
                        </TabsTrigger>
                        <TabsTrigger value="archived">
                            Архивирани
                            <Badge variant="neutral">{{ counts.archived }}</Badge>
                        </TabsTrigger>
                    </TabsList>
                </Tabs>

                <div class="relative sm:w-72 lg:ml-auto">
                    <Search class="text-muted-foreground absolute top-1/2 left-2.5 size-4 -translate-y-1/2" />
                    <Input v-model="filters.search" type="search" placeholder="Наслов" class="pl-8" aria-label="Пребарај" />
                </div>
            </CardContent>

            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead class="w-20 pl-4">Слика</TableHead>
                        <TableHead>Наслов</TableHead>
                        <TableHead>Потребен износ</TableHead>
                        <TableHead>Статус</TableHead>
                        <TableHead>Додадено</TableHead>
                        <TableHead class="pr-4"><span class="sr-only">Акции</span></TableHead>
                    </TableRow>
                </TableHeader>

                <TableBody>
                    <TableRow
                        v-for="donationNeed in donationNeeds.data"
                        :key="donationNeed.id"
                        class="cursor-pointer"
                        @click="router.visit(edit(donationNeed.id).url)"
                    >
                        <TableCell class="pl-4">
                            <img v-if="donationNeed.image_url" :src="donationNeed.image_url" alt="" class="size-10 rounded-md border object-cover" />
                        </TableCell>
                        <TableCell class="max-w-80">
                            <Link :href="edit(donationNeed.id).url" class="block truncate font-medium hover:underline" @click.stop>{{
                                donationNeed.title
                            }}</Link>
                        </TableCell>
                        <TableCell class="whitespace-nowrap tabular-nums">{{ formatMoney(donationNeed.needed_amount) }}</TableCell>
                        <TableCell>
                            <Badge :variant="donationNeed.archived_at ? 'warning' : 'success'">{{
                                donationNeed.archived_at ? 'Архивирано' : 'Активно'
                            }}</Badge>
                        </TableCell>
                        <TableCell class="text-muted-foreground whitespace-nowrap tabular-nums">{{
                            formatDateTime(donationNeed.created_at)
                        }}</TableCell>
                        <TableCell class="pr-4 text-right" @click.stop>
                            <DonationNeedArchiveButton :donation-need="donationNeed" size="sm" />
                        </TableCell>
                    </TableRow>

                    <TableRow v-if="donationNeeds.data.length === 0">
                        <TableCell colspan="6" class="h-40 text-center">
                            <div class="text-muted-foreground flex flex-col items-center gap-2">
                                <HeartHandshake class="size-6" />
                                Нема донациски потреби за овие филтри.
                            </div>
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>

            <CardContent class="border-t p-4">
                <Pagination :paginator="donationNeeds" />
            </CardContent>
        </Card>
    </div>
</template>
