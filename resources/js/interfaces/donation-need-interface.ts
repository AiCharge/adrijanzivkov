export interface DonationNeedInterface {
    id: number;
    title: string;
    description: string;
    sticky_content: string | null;
    image: string;
    image_url: string | null;
    needed_amount: number;
    archived_at: string | null;
    created_at: string;
}
