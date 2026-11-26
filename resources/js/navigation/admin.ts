import { index as donationNeeds } from '@/actions/App/Http/Controllers/Admin/DonationNeedController';
import { index as news } from '@/actions/App/Http/Controllers/Admin/NewsController';
import { index as storySubmissions } from '@/actions/App/Http/Controllers/Admin/StorySubmissionController';
import type { AdminNavigationGroupInterface } from '@/interfaces/admin-navigation-interface';
import { HeartHandshake, Inbox, Newspaper } from '@lucide/vue';

/**
 * The admin's menu: one level deep, groups named by the work.
 */
export const adminNavigation: AdminNavigationGroupInterface[] = [
    {
        key: 'content',
        label: 'Содржина',
        startsOpen: true,
        items: [
            { label: 'Донации', href: donationNeeds().url, icon: HeartHandshake },
            { label: 'Новости', href: news().url, icon: Newspaper },
        ],
    },
    {
        key: 'community',
        label: 'Заедница',
        startsOpen: true,
        items: [{ label: 'Поднесени приказни', href: storySubmissions().url, icon: Inbox }],
    },
];
