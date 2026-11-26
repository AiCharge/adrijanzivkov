<script setup lang="ts">
import { create, edit, index } from '@/actions/App/Http/Controllers/Admin/NewsController';
import PageHeader from '@/components/admin/PageHeader.vue';
import Pagination from '@/components/admin/Pagination.vue';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { useAdminFilters } from '@/composables/useAdminFilters';
import type { NewsInterface } from '@/interfaces/news-interface';
import type { PaginatedInterface } from '@/interfaces/paginated-interface';
import { formatDateTime } from '@/lib/formatters';
import { Link, router } from '@inertiajs/vue3';
import { Newspaper, Plus, Search } from '@lucide/vue';

const props = defineProps<{
    news: PaginatedInterface<NewsInterface>;
    filters: { search: string };
}>();

const filters = useAdminFilters(index().url, { search: props.filters.search });
</script>

<template>
    <div class="space-y-6">
        <PageHeader title="Новости" description="Новостите што се прикажуваат на сајтот, најновите први.">
            <template #actions>
                <Button as-child>
                    <Link :href="create().url">
                        <Plus />
                        Додај новост
                    </Link>
                </Button>
            </template>
        </PageHeader>

        <Card class="gap-0 py-0">
            <CardContent class="flex flex-col gap-3 border-b p-4 sm:flex-row sm:justify-end">
                <div class="relative sm:w-72">
                    <Search class="text-muted-foreground absolute top-1/2 left-2.5 size-4 -translate-y-1/2" />
                    <Input v-model="filters.search" type="search" placeholder="Наслов или опис" class="pl-8" aria-label="Пребарај" />
                </div>
            </CardContent>

            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead class="w-20 pl-4">Слика</TableHead>
                        <TableHead>Наслов</TableHead>
                        <TableHead>Краток опис</TableHead>
                        <TableHead>Објавено</TableHead>
                    </TableRow>
                </TableHeader>

                <TableBody>
                    <TableRow v-for="item in news.data" :key="item.id" class="cursor-pointer" @click="router.visit(edit(item.slug).url)">
                        <TableCell class="pl-4">
                            <img v-if="item.main_image_url" :src="item.main_image_url" alt="" class="size-10 rounded-md border object-cover" />
                        </TableCell>
                        <TableCell class="max-w-72">
                            <Link :href="edit(item.slug).url" class="block truncate font-medium hover:underline" @click.stop>{{ item.title }}</Link>
                        </TableCell>
                        <TableCell class="text-muted-foreground max-w-md">
                            <p class="truncate" :title="item.short_description">{{ item.short_description }}</p>
                        </TableCell>
                        <TableCell class="text-muted-foreground whitespace-nowrap tabular-nums">{{ formatDateTime(item.created_at) }}</TableCell>
                    </TableRow>

                    <TableRow v-if="news.data.length === 0">
                        <TableCell colspan="4" class="h-40 text-center">
                            <div class="text-muted-foreground flex flex-col items-center gap-2">
                                <Newspaper class="size-6" />
                                Нема новости за ова пребарување.
                            </div>
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>

            <CardContent class="border-t p-4">
                <Pagination :paginator="news" />
            </CardContent>
        </Card>
    </div>
</template>
