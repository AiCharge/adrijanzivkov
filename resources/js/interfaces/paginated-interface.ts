export interface PaginationLinkInterface {
    url: string | null;
    label: string;
    active: boolean;
}

export interface PaginatedInterface<T> {
    data: T[];
    links: PaginationLinkInterface[];
    current_page: number;
    last_page: number;
    per_page: number;
    total: number;
    from: number | null;
    to: number | null;
    prev_page_url: string | null;
    next_page_url: string | null;
}
