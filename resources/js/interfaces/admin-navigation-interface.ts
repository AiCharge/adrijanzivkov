import type { Component } from 'vue';

export interface AdminNavigationItemInterface {
    label: string;
    href: string;
    icon: Component;
}

/**
 * One group of the admin's menu. `startsOpen` is how it shows until the admin opens or folds it.
 */
export interface AdminNavigationGroupInterface {
    key: string;
    label: string;
    startsOpen: boolean;
    items: AdminNavigationItemInterface[];
}
